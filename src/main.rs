mod config;
mod corridor;
mod helpers;
mod logging;
mod maze;
mod patcher;
mod qol_hacks;
mod rebalance;
mod colors;
mod seed;

use rand_chacha::ChaCha8Rng;
use rand_seeder::Seeder;

use crate::config::{BadIdeas, ColorStrategy, Config, CorridorConfig, HueOptions, QOLHacks, SaturationOptions};
use crate::patcher::Patcher;

fn generate(patcher: &mut Patcher, cfg: &Config) {
    let mut rng: ChaCha8Rng = Seeder::from(&cfg.seed).make_rng();
    corridor::shuffle_corridor_components(patcher, cfg, &mut rng);
    rebalance::handle_rebalance(patcher, cfg, &mut rng);
    maze::shuffle_minibosses::shuffle_minibosses(patcher,cfg, &mut rng);
    let items = maze::items::item_generator::ItemGenerator::prepare_items(patcher, 5, 5, 4, 9, 10, 6, true, 5, 5, 3, 5, cfg.log, &mut rng);
    
    
    let map = maze::generator::Generator.run(
        items.0,
        items.1,
        items.2,
        cfg.secret,
        18,
        25,
        3,
        0,
        false,
        6,
        3,
        10,
        cfg.log,
        &mut rng
    );
    match map {
        Ok(map) => {
            if cfg.log
            {
                map.draw_exits();
            }
            let maphex = map.write_hex(cfg.log);
            patcher.add_change(&maphex, "14A7E");
        }
        Err(e) => {
            panic!("{}",e);
        }
    }
    
    colors::patch_themes::patch_all(cfg,patcher,&mut Seeder::from(&cfg.seed).make_rng());
    

    qol_hacks::handle_qol_hacks(patcher, cfg);

}

fn main() {
    let writefiles = true;
    let secret = false;

    let mut patcher = Patcher::new();

    let corridor_config = CorridorConfig {
        shuffle_skies: true,
        shuffle_ground: false, //currently jank
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
    let bad_ideas = BadIdeas {
    };



    let rng_seed = seed::make_seed();


    let hue_options  = HueOptions{rotate_hue: true, flip_saturation: SaturationOptions::Safe};
    let cfg = Config {
        corridor_config,
        qol_hacks,
        bad_ideas,
        color_strategy: ColorStrategy::ColorTheory(hue_options),
        rebalance_bosses: true,
        randomize_boss_health: true,
        secret,
        log: true,
        seed: rng_seed,
    };

    generate(&mut patcher, &cfg);
    seed::write_seed(&mut patcher,&cfg);


    if writefiles {
        let rawdata = if secret {
            std::fs::read("./sourceroms/secret4rando.nes").unwrap()
        } else {
            std::fs::read("./sourceroms/tgl.nes").unwrap()
        };

        let rom = hex::encode(rawdata);
        //println!("ROM data: {}", rom);

        let filetag = if secret { "SECRET" } else { "TGL" };

        let rom_filename = "./output/1brokian.nes";
        let rom_filename2 = format!("./output/{}-{}-{}.nes", filetag, chrono::Local::now().format("%Y-%m-%d"), cfg.seed);

        patcher.write_rom(&rom_filename, &rom);
        patcher.write_rom(&rom_filename2, &rom);

        /*// Write IPS data to a .ips file
        let byte_array = b"your_ips_data_here"; // Replace with actual IPS data
        let ips_filename = format!(
            "./output/{}-{}.ips",
            filetag,
            chrono::Local::now().format("%Y-%m-%d-%H-%M-%S")
        );
        let mut ips_file = File::create(ips_filename).expect("Unable to create IPS file");
        ips_file
            .write_all(byte_array)
            .expect("Unable to write IPS data");

        // Write CSV data to a .csv file
        let csv_data = "your_csv_data_here"; // Replace with actual CSV data
        let csv_filename = format!(
            "./output/{}-{}.csv",
            filetag,
            chrono::Local::now().format("%Y-%m-%d-%H-%M-%S")
        );
        let mut csv_file = File::create(csv_filename).expect("Unable to create CSV file");
        csv_file
            .write_all(csv_data.as_bytes())
            .expect("Unable to write CSV data");*/
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_seed_consistancy() {
        use super::*;
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
            secret: false,
            log: true,
            seed: "TestSeed".to_string(),
        };

        let mut p1 = Patcher::new();

        generate(&mut p1, &cfg);

        let ips1 = p1.build_ips();

        let mut p2 = Patcher::new();

        generate(&mut p2, &cfg);

        let ips2 = p2.build_ips();
        assert_eq!(ips1, ips2);
    }

    fn test_100_generations() {
        use super::*;
        for i in 0..1000
        {
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
                secret: false,
                log: false,
                seed: rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(10)
                    .map(char::from)
                    .collect(),
            };
            let mut p1 = Patcher::new();

            generate(&mut p1, &cfg);
            print!("{}",i);
        }

    }
}
