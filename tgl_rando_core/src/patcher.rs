pub struct Change {
    hex: String,
    offset: String,
}

impl Change {
    pub fn new(hex: &str, offset: &str) -> Self {
        Self {
            hex: hex.to_string(),
            offset: offset.to_string(),
        }
    }
}

pub struct Patcher {
    changes: Vec<Change>,
}

impl Patcher {
    pub fn new() -> Self {
        Self {
            changes: Vec::new(),
        }
    }

    pub fn add_change(&mut self, hex: &str, offset: &str) {
        let padded = if hex.len() % 2 == 1 {
            "0".to_string() + hex
        } else {
            hex.to_string()
        };

        let change = Change::new(padded.as_str(), offset);
        self.changes.push(change);
    }

    pub fn build_ips(&self) -> Vec<u8> {
        let mut byte_array: Vec<u8> = Vec::new();
        byte_array.extend_from_slice(&hex::decode("5041544348").unwrap()); // add header

        for change in &self.changes {
            let offset = &change.offset;

            let offset_bytes = hex::decode(crate::helpers::pad_hex(offset, 6)).unwrap();
            byte_array.extend_from_slice(&offset_bytes);

            let change_hex = &change.hex;
            let change_bytes = hex::decode(change_hex).unwrap();
            let length = (change_bytes.len()) as u16;
            let length_bytes = length.to_be_bytes();
            byte_array.extend_from_slice(&length_bytes);
            byte_array.extend_from_slice(&change_bytes);
        }

        byte_array.extend_from_slice(&hex::decode("454f46").unwrap()); // add end of file

        byte_array
    }

    pub fn write_rom(&self, filename: &str, source_data: &str) {
        let mut patched = source_data.to_owned();

        for change in &self.changes {
            let offset = u32::from_str_radix(&change.offset, 16).unwrap() * 2;
            let change_hex = &change.hex;
            patched.replace_range(
                offset as usize
                    ..(offset + (change_hex.len() as u32)).min(patched.len() as u32) as usize,
                change_hex,
            );
        }

        std::fs::write(filename, hex::decode(patched).unwrap()).expect("Unable to write file");
    }

    pub fn patch_u8_vec(&self, source_data: &Vec<u8>) -> Vec<u8>{
        let mut patched = source_data.clone();

        for change in &self.changes {
            let offset = usize::from_str_radix(&change.offset, 16).unwrap();
            let change_bytes = hex::decode(&change.hex).unwrap();
            let end = offset + change_bytes.len();
            patched.splice(offset..end, change_bytes.iter().cloned());
        }
        patched
    }


}

#[cfg(test)]
mod tests {
    //use crate::patcher::Patcher;

    use super::*;
    #[test]
    fn test_add_patch() {
        let mut pat = Patcher::new();
        pat.add_change("0f", "18bbd");
        pat.add_change("0f", "894c");
        assert_eq!(pat.changes[0].hex, "0f");
        assert_eq!(pat.changes[1].hex, "0f");
        assert_eq!(pat.changes[0].offset, "18bbd");
        assert_eq!(pat.changes[1].offset, "894c");
    }

    #[test]
    fn test_write_ips() {
        let mut pat = Patcher::new();
        pat.add_change("0f", "18bbd");
        pat.add_change("0f", "894c");
        let export = pat.build_ips();
        assert_eq!(
            hex::encode(export),
            "5041544348018bbd00010f00894c00010f454f46"
        );
    }
}
