mod colors;
pub mod config;
mod corridor;
mod helpers;
mod logging;
mod maze;
mod music;
pub mod patcher;
mod qol_hacks;
mod rebalance;
pub mod seed;
pub mod tgl_error;

use rand_chacha::ChaCha8Rng;
use rand_seeder::Seeder;

use crate::config::{Config, SaturationOptions};
use crate::patcher::Patcher;
use crate::tgl_error::TGLError;

pub fn generate(patcher: &mut Patcher, cfg: &Config) -> Result<(), TGLError> {
    let mut rng: ChaCha8Rng = Seeder::from(&cfg.seed).make_rng();
    corridor::shuffle_corridor_components(patcher, cfg, &mut rng)?;
    rebalance::handle_rebalance(patcher, cfg, &mut rng);
    if cfg.boss_config.shuffle_bosses {
        maze::shuffle_minibosses::shuffle_minibosses(patcher, cfg, &mut rng);
    }
    let item_library =
        maze::items::item_generator::ItemGenerator::prepare_items(patcher, cfg, &mut rng);

    let map = maze::generator::Generator.run(&item_library?, cfg, &mut rng);
    match map {
        Ok(map) => {
            if cfg.log {
                map.draw_exits()?;
            }
            let maphex = map.write_hex(cfg.log)?;
            patcher.add_change(&maphex, "14A7E");
        }
        Err(e) => return Err(e),
    }

    colors::patch_themes::patch_all(cfg, patcher, &mut Seeder::from(&cfg.seed).make_rng())?;
    music::patch_music(patcher, cfg, &mut Seeder::from(&cfg.seed).make_rng());

    qol_hacks::handle_qol_hacks(patcher, cfg);
    seed::write_seed(patcher, cfg);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_seed_consistancy() {
        use super::*;

        let cfg = Config {
            seed: "TestSeed".to_string(),
            ..Config::default()
        };

        let mut p1 = Patcher::new();

        generate(&mut p1, &cfg).unwrap();

        let ips1 = p1.build_ips().unwrap();

        let mut p2 = Patcher::new();

        generate(&mut p2, &cfg).unwrap();

        let ips2 = p2.build_ips().unwrap();
        assert_eq!(ips1, ips2);
    }

    /*fn test_100_generations() {
        use super::*;
        for i in 0..1000 {
            let corridor_config = CorridorConfig {
                shuffle_skies: true,
                shuffle_ground: true,
                shuffle_corridors: true,
                shuffle_bosses: true,
                shuffle_final_boss: true,
            };
            let qol_hacks = QOLHacks {
                faster_starting_fire: true,
                fix_hyper_laser: true,
                enemy_erasers_unlocked_from_start: true,
                remove_flash: true,
            };

            let cfg = Config {
                corridor_config,
                qol_hacks,
                rebalance_bosses: true,
                randomize_boss_health: true,
                log: false,
                seed: rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(10)
                    .map(char::from)
                    .collect(),
            };
            let mut p1 = Patcher::new();

            generate(&mut p1, &cfg);
            print!("{}", i);
        }
    }*/
}
