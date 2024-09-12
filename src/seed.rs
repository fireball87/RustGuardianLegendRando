use crate::config::Config;
use crate::patcher::Patcher;
use rand::Rng;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            0123456789";
const PASSWORD_LEN: usize = 11;

pub fn write_seed(patcher: &mut Patcher, config: &Config) {
    //press start -> 0x855, allows A-Z,1-9, some other chars i don't know how to access yet
    //BRODERBUND SOFTWARE INC. 24 characters
    if config.seed.is_ascii()
        && config.seed.len() <= PASSWORD_LEN
        && config.seed.chars().all(|c| CHARSET.contains(&(c as u8)))
    {
        let seed = config.seed.clone();

        let seed_hex: String = seed
            .as_bytes()
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect();
        patcher.add_change("2020202020202020202020", "0855");

        patcher.add_change(&seed_hex, "0855");

        //cpw by fireball
        patcher.add_change("20404649524542414C4C20", "069C");
        //nintendo of america inc. 24 characters
        patcher.add_change("202020202020202020202020202020202020202020202020", "06AA");
        patcher.add_change("534545445D", "06AE");
        patcher.add_change(&seed_hex, "06B3");
    }
}

pub fn make_seed() -> String {
    let mut rng = rand::thread_rng();

    let rng_seed: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    rng_seed
}
