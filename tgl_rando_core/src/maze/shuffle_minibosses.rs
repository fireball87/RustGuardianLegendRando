use crate::config::Config;
use crate::Patcher;
use rand::Rng;
use rand_chacha::ChaCha8Rng;

pub fn shuffle_minibosses(patcher: &mut Patcher, cfg: &Config, rng: &mut ChaCha8Rng) {
    // 22 minibosses
    // 3 * a number between 0 and 11, 12 if allow_missingno
    let monster_values = if cfg.boss_config.allow_missingno {
        12
    } else {
        11
    };

    let mut datum = String::new();
    for _ in 0..22 {
        let value = rng.gen_range(0..=monster_values) * 3;
        datum.push_str(&format!("{:02X}", value));
    }

    patcher.add_change(&datum, "1669D");
}
