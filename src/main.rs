mod config;
mod corridor;
mod logging;
mod patcher;
mod rebalance;
mod rebalance;

use crate::config::{Config, CorridorConfig};
use crate::patcher::Patcher;

fn main() {
    let writefiles = true;
    let secret = false;

    let mut patcher = Patcher::new();

    let c_cfg = CorridorConfig {
        shuffle_skies: true,
        shuffle_ground: true,
        shuffle_corridors: true,
        shuffle_bosses: true,
        shuffle_final_boss: true,
    };

    let cfg = Config {
        corridor_config: c_cfg,
        log: true,
    };

    corridor::shuffle_corridor_components(&mut patcher, cfg);

    if writefiles {
        let rawdata = if secret {
            std::fs::read("./sourceroms/secret4rando.nes").unwrap()
        } else {
            std::fs::read("./sourceroms/tgl.nes").unwrap()
        };

        let rom = hex::encode(rawdata);
        //println!("ROM data: {}", rom);

        let filetag = if secret { "secret" } else { "tgl" };

        let rom_filename = format!(
            "./output/{}-{}.nes",
            filetag,
            chrono::Local::now().format("%Y-%m-%d-%H-%M-%S")
        );

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
