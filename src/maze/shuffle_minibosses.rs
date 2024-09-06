use rand::Rng;
use rand_chacha::ChaCha8Rng;
use crate::config::Config;
use crate::Patcher;

pub fn shuffle_minibosses(patcher: &mut Patcher, config: &Config, rng: &mut ChaCha8Rng) {
    // 22 minibosses
    // 3 * a number between 0 and 11, 12 if allow_missingno
    let allow_missingno = false;
    let monstervalues = if allow_missingno { 12 } else { 11 };

    let mut datum = String::new();
    for _ in 0..22 {
        let value = rng.gen_range(0..monstervalues * 3);
        datum.push_str(&format!("{:X}", value));
    }

    patcher.add_change(&datum, "1669D");
}
