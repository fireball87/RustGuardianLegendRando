use crate::config::*;
use crate::patcher::Patcher;

mod shuffle_bosses;
mod shuffle_corridors;
mod shuffle_ground;
mod shuffle_skies;

pub fn shuffle_corridor_components(patcher: &mut Patcher, config: Config) {
    if config.corridor_config.shuffle_skies {
        shuffle_skies::shuffle_skies(patcher);
    }

    let shuffled_bosses = if config.corridor_config.shuffle_bosses {
        Some(shuffle_bosses::randomize_bosses(
            patcher,
            config.corridor_config.shuffle_final_boss,
        ))
    } else {
        None
    };

    if shuffled_bosses.is_some() || config.corridor_config.shuffle_corridors {
        shuffle_corridors::shuffle_corridors(
            patcher,
            config.corridor_config.shuffle_corridors,
            shuffled_bosses,
            config.log,
        )
    }

    /*if(shuffled_bosses.is_some()||config.corridor_config.shuffle_ground) {
        shuffle_ground::shu
    }*/
}
