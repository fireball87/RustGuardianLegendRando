mod config;
mod corridor;
mod helpers;
mod logging;
mod maze;
mod patcher;
mod qol_hacks;
mod rebalance;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand_seeder::{Seeder, SipHasher};

use crate::config::{Config, CorridorConfig, QOLHacks};
use crate::patcher::Patcher;
use rand::{distributions::Alphanumeric, Rng, SeedableRng};

fn generate(patcher: &mut Patcher, cfg: &Config) {
    let mut rng: ChaCha8Rng = Seeder::from(&cfg.seed).make_rng();
    qol_hacks::handle_qol_hacks(patcher, cfg);
    corridor::shuffle_corridor_components(patcher, cfg, &mut rng);
    rebalance::handle_rebalance(patcher, cfg, &mut rng);
}

fn main() {
    let writefiles = true;
    let secret = false;

    let mut patcher = Patcher::new();

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

    let rng_seed = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    let cfg = Config {
        corridor_config,
        qol_hacks,
        rebalance_bosses: true,
        randomize_boss_health: true,
        log: true,
        seed: rng_seed,
    };

    generate(&mut patcher, &cfg);

    if writefiles {
        let rawdata = if secret {
            std::fs::read("./sourceroms/secret4rando.nes").unwrap()
        } else {
            std::fs::read("./sourceroms/tgl.nes").unwrap()
        };

        let rom = hex::encode(rawdata);
        //println!("ROM data: {}", rom);

        let filetag = if secret { "secret" } else { "tgl" };

        let rom_filename = format!("./output/{}-{}.nes", filetag, cfg.seed);

        patcher.write_rom(&rom_filename, &rom);
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
}
