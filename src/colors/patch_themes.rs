use crate::SaturationOptions;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use crate::colors::patch_themes::ColorTheory::{Complementary, Monochrome, Triad};
use crate::config::{ColorStrategy, Config, HueOptions};
use crate::patcher::Patcher;

#[derive(PartialEq, Eq)]
enum ThemeSet {
    All,
    Vanilla,
    Crafted,
    ColorTheory(ColorTheory)
}

#[derive(PartialEq, Eq)]
enum ColorTheory {
    Monochrome,
    Complementary,
    Triad,
}
struct Patch {
    address: &'static str,
    hex_code: &'static str,
}

struct PatchSet {
    name: &'static str,
    theme_set: Vec<ThemeSet>,
    patches: Vec<Patch>,
    saturation_flip_safe: bool,
}
pub fn patch_all(cfg: &Config, patcher: &mut Patcher, rng: &mut ChaCha8Rng){
    let areas = [get_c0(), get_a0(), /*get_a1(), get_a3(), get_a5(), get_a7(), get_a9()*/];

    for area in areas {
        pick_level(area.0, area.1, rng, patcher, cfg)
    }
}

fn pick_level(level: &str, area:Vec<PatchSet>, rng: &mut ChaCha8Rng, patcher: &mut Patcher, cfg: &Config){
        let strategy= &cfg.color_strategy;
        match strategy {
            ColorStrategy::Vanilla(hue) => {
                let index = 0;
                let selected = &area[index];
                patch_single_area(level, patcher,rng,selected,hue, cfg);

            }
            ColorStrategy::All(hue ) => {
                let index = rng.gen_range(0..area.len());
                let selected = &area[index];
                patch_single_area(level,patcher,rng,selected,hue,cfg);
            }
            ColorStrategy::Random => {
                let index = 0;
                let selected = &area[index];
                for values in &selected.patches {
                    patcher.add_change(&*get_ran_palette(rng), values.address);
                }
            }
            ColorStrategy::ColorTheory(hue) => {
                let filtered: Vec<&PatchSet> = area.iter().filter( |p| p.theme_set.iter().any(|a|matches!(a,ThemeSet::ColorTheory(_)))).collect();
                let index = rng.gen_range(0..filtered.len());
                let selected = filtered[index];
                patch_single_area(level,patcher,rng,selected,hue, cfg);
            }
        }
}

fn patch_single_area(level: &str, patcher: &mut Patcher,rng: &mut ChaCha8Rng, selected: &PatchSet, hue_options: &HueOptions, cfg: &Config) {
    let flip = rng.gen_range(0..=1);
    let shift_distance = rng.gen_range(0..=11);
    let flip_allowed;
    let flip_safe;
    match hue_options.flip_saturation{
        SaturationOptions::None => {
            flip_allowed = false;
            flip_safe = false;
        }
        SaturationOptions::Safe => {
            flip_allowed = true;
            flip_safe = true;

        }
        SaturationOptions::All => {
            flip_allowed = true;
            flip_safe = true;
        }
    }
    let mut should_flip = false;
    if flip == 1 && flip_allowed {
        if !flip_safe || selected.saturation_flip_safe
        {
            should_flip = true;
        }
    }
    
    let colorshift;
    match hue_options.rotate_hue {
        true => {colorshift = format!("{}",shift_distance)}
        false => {colorshift = "false".to_string()}
    }
    
    if cfg.log {
        println!("picked {} theme for area {} shifted {} flip:{}",selected.name,level,colorshift,should_flip)
    }
    
    
    
    
    for values in &selected.patches {

        
        let mut shifted;
        match hue_options.rotate_hue{
            true => {
                shifted = shift_all_colors(values.hex_code, shift_distance);
            }
            false => {shifted = values.hex_code.to_string()}
        }
        

            
        if should_flip {
            shifted = flip_all_saturation(&shifted);
        }


        patcher.add_change(&shifted, values.address);
    }
}

fn flip_all_saturation(colors: &str) -> String{
    let v1 = flip_single_saturation(&colors[..2]);
    let v2 = flip_single_saturation(&colors[2..4]);
    let v3 = flip_single_saturation(&colors[4..]);

    format!("{}{}{}",v1,v2,v3)
}
fn flip_single_saturation(color: &str) -> String {
        let mut chars = color.chars();
        let c1 = &chars.next().unwrap();
        let c2 = &chars.next().unwrap();
        let saturation = ['0','1','2','3'];
        let hue = ['1','2','3','4','5','6','7','8','9','A','B','C'];
        if saturation.contains(c1) && hue.contains(c2) {
            match c1 {
                '0' => format!("{}{}", '3', c2),
                '1' => format!("{}{}", '2', c2),
                '2' => format!("{}{}", '1', c2),
                '3' => format!("{}{}", '0', c2),
    
                default => color.to_string()
            }
        }
        else { {
            color.to_string()
        } 
    }
    
}
fn shift_all_colors(colors: &str, distance: usize) -> String{
    let v1 = shift_single_color_hue(&colors[..2],distance);
    let v2 = shift_single_color_hue(&colors[2..4], distance);
    let v3 = shift_single_color_hue(&colors[4..],distance);

    format!("{}{}{}",v1,v2,v3)

}
fn shift_single_color_hue(color: &str, distance: usize) -> String {
    let mut chars = color.chars();
    let c1 = &chars.next().unwrap();
    let c2 = &chars.next().unwrap();
    let saturation = ['0','1','2','3'];
    let hue = ['1','2','3','4','5','6','7','8','9','A','B','C'];
    if saturation.contains(c1) && hue.contains(c2) {
        let index = hue.iter().position(|&c| c == *c2).unwrap();
        let new_index = (index + distance) % hue.len();

        format!("{}{}",c1,hue[new_index])
    }
    else {
        color.to_string()
    }
}



fn get_c0() -> (&'static str,Vec<PatchSet>){
    //patch c0
    // 0x17344 -> 1c,0c,16
    // used by background floor in c0
    // 0x1733E -> 11,01,16
    // used by foreground floor in c0
    // 0x17341 -> 22,12,16
    // used by eye cannons and stuff
    ("c0",vec![
        /*PatchSet{
            name: "Vanilla",
            theme_set: vec![ThemeSet::All, ThemeSet::Vanilla],
            saturation_flip_safe: false,
            patches: vec![
                Patch{address: "17344", hex_code: "1C0C16"}, 
                Patch{address: "1733E", hex_code: "110116"}, 
                Patch{address: "17341", hex_code: "221216"}
            ],
        },
        PatchSet{
            name: "Purple",
            theme_set: vec![ThemeSet::All,ThemeSet::Crafted],
            saturation_flip_safe: false,
            patches: vec![
                Patch{address: "17344", hex_code: "140326"},
                Patch{ address: "1733E", hex_code: "241326"},
                Patch{ address: "17341", hex_code: "130307"}
            ],
        },
        PatchSet{
            name: "Grayscale",
            theme_set: vec![ThemeSet::All,ThemeSet::Crafted],
            saturation_flip_safe: true,
            patches: vec![
                Patch{ address: "17344", hex_code: "3D2D27"},
                Patch{ address: "1733E", hex_code: "3D2D27"},
                Patch{ address: "17341", hex_code: "2D3D27"}
            ],
        },
        PatchSet{
            name: "Icy",
            theme_set: vec![ThemeSet::All,ThemeSet::Crafted],
            saturation_flip_safe: false,
            patches: vec![
                Patch{ address: "1733E", hex_code: "3C2C32"},
                Patch{ address: "17341", hex_code: "1C3C22"},
                Patch{ address: "17344", hex_code: "1C0C31"},
            ],
        },
        PatchSet{
            name: "Burnt",
            theme_set: vec![ThemeSet::All,ThemeSet::Crafted, ThemeSet::ColorTheory(Monochrome)],
            saturation_flip_safe: false,
            patches: vec![
                Patch{ address: "1733E", hex_code: "261707"},
                Patch{ address: "17341", hex_code: "061628"},
                Patch{ address: "17344", hex_code: "070801"},
            ],
        },
        PatchSet{
            name: "GIJoe",
            theme_set: vec![ThemeSet::All,ThemeSet::Crafted, ThemeSet::ColorTheory(ColorTheory::Triad)],
            saturation_flip_safe: false,
            patches: vec![
                Patch{ address: "1733E", hex_code: "250521"},
                Patch{ address: "17341", hex_code: "120215"},
                Patch{ address: "17344", hex_code: "180835"},
            ],
        },
        PatchSet{
            name: "AlmostNormal",
            theme_set: vec![ThemeSet::All,ThemeSet::Crafted, ThemeSet::ColorTheory(ColorTheory::Complementary)],
            saturation_flip_safe: false,
            patches: vec![
                Patch{ address: "1733E", hex_code: "120129"},
                Patch{ address: "17341", hex_code: "021216"},
                Patch{ address: "17344", hex_code: "160819"},
            ],
        },*/
        PatchSet{
            name: "Shadow", 
            theme_set: vec![ThemeSet::All,ThemeSet::Crafted],
            saturation_flip_safe: false,
            patches: vec![
                Patch{ address: "1733E", hex_code: "010D38"},
                Patch{ address: "17341", hex_code: "010D38"},
                Patch{ address: "17344", hex_code: "0D0D08"},
            ],
        },
    ])
}



fn get_a0() -> (&'static str,Vec<PatchSet>){
    ("a0",vec![
        PatchSet{
            name: "Vanilla",
            theme_set: vec![ThemeSet::All, ThemeSet::Vanilla],
            saturation_flip_safe:true,
            patches: vec![
                Patch{ address: "17326", hex_code: "301A0A"},
                Patch{ address: "1732F", hex_code: "301000"},
            ],
        },
        PatchSet{
            name: "Sand",
            theme_set: vec![ThemeSet::All, ThemeSet::Crafted, ThemeSet::ColorTheory(Monochrome)],
            saturation_flip_safe:true,
            patches: vec![
                Patch{ address: "17326", hex_code: "362838"},
                Patch{ address: "1732F", hex_code: "362717"},
            ],
        },
        PatchSet{
            name: "Barbie",
            theme_set: vec![ThemeSet::All, ThemeSet::Crafted],
            saturation_flip_safe:true,
            patches: vec![
                Patch{ address: "17326", hex_code: "241504"},
                Patch{ address: "1732F", hex_code: "243424"},
            ],
        },
        PatchSet{
            name: "Dank",
            theme_set: vec![ThemeSet::All, ThemeSet::Crafted],
            saturation_flip_safe:false,
            patches: vec![
                Patch{ address: "17326", hex_code: "0A1D08"},
                Patch{ address: "1732F", hex_code: "0A0B0C"},
            ],
        },
        PatchSet{
            name: "Grandmas",
            theme_set: vec![ThemeSet::All, ThemeSet::Crafted, ThemeSet::ColorTheory(Triad)],
            saturation_flip_safe:true,
            patches: vec![
                Patch{ address: "17326", hex_code: "3A0717"},
                Patch{ address: "1732F", hex_code: "3A1A03"},
            ],
        },
        PatchSet{
            name: "Golf",
            theme_set: vec![ThemeSet::All, ThemeSet::Crafted, ThemeSet::ColorTheory(Complementary)],
            saturation_flip_safe:true,
            patches: vec![
                Patch{ address: "17326", hex_code: "390312"},
                Patch{ address: "1732F", hex_code: "392719"},
            ],
        },


    ])
}

/*fn get_a3() -> Vec<Vec<Patch>>{
    vec![
        //vanilla
        vec![
            //1. used for flowers, 
            // first 2 colors should match 2
            // Final value should match final value of 3
            Patch{address: "1735C", hex_code: "30160A"},
            //2. demon plant, 
            // first 2 colors should match 1
            // last color should be close
            Patch{address: "1735F", hex_code: "301606"},
            //3. background
            Patch{address: "17362", hex_code: "2A1A0A"},
        ],
        //fall
        vec![
            //1. used for flowers, 
            // first 2 colors should match 2
            // Final value should match final value of 3
            Patch{address: "1735C", hex_code: "281707"},
            //2. demon plant, 
            // first 2 colors should match 1
            // last color should be close
            Patch{address: "1735F", hex_code: "271706"},
            //3. background
            Patch{address: "17362", hex_code: "261707"},
        ],
    ]
}


fn get_a9() -> Vec<Vec<Patch>>{
    vec![
        //vanilla
        vec![
            //1. donno 
            Patch{address: "1734A", hex_code: "271707"},
            //2. donno
            Patch{address: "1734D", hex_code: "37170F"},
            //3. background
            Patch{address: "17350", hex_code: "281808"},
        ],
    ]
}

fn get_a7() -> Vec<Vec<Patch>>{
    vec![
        //vanilla
        vec![
            //1. eyeballs seem to be it 
            Patch{address: "17353", hex_code: "302112"},
            //2. i think this is the blue plants
            Patch{address: "17356", hex_code: "221205"},
            //3. background
            Patch{address: "17359", hex_code: "250605"},
        ],
    ]
}

fn get_a5() -> Vec<Vec<Patch>>{
    vec![
        //vanilla
        vec![
            //1. blue orb background 
            Patch{address: "1736E", hex_code: "311202"},
            //2. volcano center, also it seems like save room tiles
            Patch{address: "17371", hex_code: "351505"},
            //3. background
            Patch{address: "17374", hex_code: "301000"},
            // the floor hexes in the overworld are shared with
            // the crates for some reason, we still load volcano color
            // does something use that?
        ],
    ]
}


fn get_a1() -> Vec<Vec<Patch>>{
    vec![
        //vanilla
        vec![
            //1. used for grass, middle almost certainly must match
            Patch{address: "17365", hex_code: "2A1C1A"},
            //2. firing ground plants, middle color should match ground middle
            //last color is used for the center of the bubble spawners
            Patch{address: "17368", hex_code: "301C16"},
            //3. ground colors, middle should match 2 middle
            Patch{address: "1736B", hex_code: "2C1C0C"},
        ],
        //deep blue
        vec![
            //1. used for grass, middle almost certainly must match
            Patch{address: "17365", hex_code: "180104"},
            //2. firing ground plants, middle color should match ground middle
            //last color is used for the center of the bubble spawners
            Patch{address: "17368", hex_code: "140127"},
            //3. ground colors, middle should match 2 middle
            Patch{address: "1736B", hex_code: "110102"},
        ],
    ]
}*/

fn get_ran_palette(rng: &mut ChaCha8Rng) -> String{
    let colors = 
        vec![
            "01","02","03","04","05","06","07","08","09","0A","0B","0C",
            "11","12","13","14","15","16","17","18","19","1A","1B","1C",
            "21","22","23","24","25","26","27","28","29","2A","2B","2C",
            "31","32","33","34","35","36","37","38","39","3A","3B","3C",
            "1D","2D","3D","30"
        ];
    let c1 = rng.gen_range(0..colors.len());
    let c2 = rng.gen_range(0..colors.len());
    let c3 = rng.gen_range(0..colors.len());

    format!("{}{}{}",c1,c2,c3)

}