pub fn pad_hex(hex: &str, digits: usize) -> String {
    let mut padded_hex = hex.to_string();
    while padded_hex.len() < digits {
        padded_hex = format!("0{}", padded_hex);
    }
    padded_hex
}
