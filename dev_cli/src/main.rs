use tgl_rando_core::{generate, seed, patcher::Patcher, config::*};

fn main() {
    let writefiles = true;

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

    let rng_seed = seed::make_seed();

    let hue_options = HueOptions {
        rotate_hue: true,
        flip_saturation: SaturationOptions::Safe,
    };
    let cfg = Config {
        corridor_config,
        qol_hacks,
        color_strategy: ColorStrategy::ColorTheory(hue_options),
        rebalance_bosses: true,
        randomize_boss_health: true,
        log: true,
        seed: rng_seed,
    };

    generate(&mut patcher, &cfg);
    seed::write_seed(&mut patcher, &cfg);

    if writefiles {
        let rawdata = std::fs::read("./sourceroms/tgl.nes").unwrap();

        let rom = hex::encode(rawdata);
        //println!("ROM data: {}", rom);

        let filetag = "TGL";

        let rom_filename = "./output/1brokian.nes";
        let rom_filename2 = format!(
            "./output/{}-{}-{}.nes",
            filetag,
            chrono::Local::now().format("%Y-%m-%d"),
            cfg.seed
        );

        patcher.write_rom(rom_filename, &rom);
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
