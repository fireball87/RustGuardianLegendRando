use tgl_rando_core::{generate, seed, patcher::Patcher, config::*};

fn main() {
    let writefiles = true;

    let mut patcher = Patcher::new();
    
    let cfg = Config::default();

    generate(&mut patcher, &cfg);

    if writefiles {
        let rawdata = std::fs::read("./sourceroms/tgl.nes").unwrap();

        let rom = hex::encode(rawdata);
        //println!("ROM data: {}", rom);

        let filetag = "TGL";
        
        let rom_filename = "./output/1brokian.nes";
        let rom_filename2 = format!(
            "./output/{}-{}.nes",
            filetag,
            cfg.seed
        );

        patcher.write_rom(rom_filename, &rom);
        patcher.write_rom(&rom_filename2, &rom);
    }
}
