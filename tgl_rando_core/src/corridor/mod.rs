use crate::config::*;
use crate::patcher::Patcher;
use rand_chacha::ChaCha8Rng;

mod shuffle_bosses;
mod shuffle_corridors;
mod shuffle_ground;
mod shuffle_skies;

pub fn shuffle_corridor_components(patcher: &mut Patcher, config: &Config, rng: &mut ChaCha8Rng) {
    if config.corridor_config.shuffle_skies {
        shuffle_skies::shuffle_skies(patcher, rng);
    } else {
        //fix the c16 bug
        patcher.add_change("7F", "1301b");
    }

    let shuffled_bosses = if config.boss_config.shuffle_bosses {
        Some(shuffle_bosses::randomize_bosses(
            patcher,
            config.boss_config.shuffle_final_boss,
            rng,
        ))
    } else {
        None
    };

    if shuffled_bosses.is_some() || config.corridor_config.shuffle_corridors {
        shuffle_corridors::shuffle_corridors(
            patcher,
            config.corridor_config.shuffle_corridors,
            &shuffled_bosses,
            config.log,
            rng,
        )
    }

    if shuffled_bosses.is_some() || config.corridor_config.shuffle_ground {
        shuffle_ground::shuffle_corridor_internals(
            patcher,
            config.corridor_config.shuffle_ground,
            &shuffled_bosses,
            rng,
        )
    }
}
