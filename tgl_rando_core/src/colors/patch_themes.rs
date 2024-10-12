use crate::colors::patch_themes::ColorTheory::{Complementary, Monochrome, Triad};
use crate::config::ColorStrategy::Vanilla;
use crate::config::{ColorStrategy, Config, HueOptions};
use crate::patcher::Patcher;
use crate::SaturationOptions;
use rand::Rng;
use rand_chacha::ChaCha8Rng;

#[derive(PartialEq, Eq)]
enum ThemeSet {
    All,
    Vanilla,
    Crafted,
    ColorTheory(ColorTheory),
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
pub fn patch_all(cfg: &Config, patcher: &mut Patcher, rng: &mut ChaCha8Rng) {
    if let Vanilla(hue) = &cfg.color_options.color_strategy {
        if !hue.rotate_hue && hue.flip_saturation == SaturationOptions::None {
            return;
        }
    }
    move_a5_floor_color_to_volcano(patcher);

    let mut areas = vec![
        get_c0(),
        get_a0(),
        get_a1(),
        get_a3(),
        get_a5(),
        get_a7(),
        get_a9(),
    ];

    let foreground = vec![
        get_foreground_title(),
        get_foreground_enemies(),
        get_final_boss_foreground(),
    ];

    if cfg.color_options.include_foreground {
        areas.extend(foreground);
    };

    for area in areas {
        pick_level(area.0, area.1, rng, patcher, cfg)
    }
}

fn pick_level(
    level: &str,
    area: Vec<PatchSet>,
    rng: &mut ChaCha8Rng,
    patcher: &mut Patcher,
    cfg: &Config,
) {
    let strategy = &cfg.color_options.color_strategy;
    match strategy {
        ColorStrategy::Vanilla(hue) => {
            let index = 0;
            let selected = &area[index];
            patch_single_area(level, patcher, rng, selected, hue, cfg);
        }
        ColorStrategy::All(hue) => {
            let index = rng.gen_range(0..area.len());
            let selected = &area[index];
            patch_single_area(level, patcher, rng, selected, hue, cfg);
        }
        ColorStrategy::Random => {
            let index = 0;
            let selected = &area[index];
            for values in &selected.patches {
                patcher.add_change(&get_ran_palette(rng), values.address);
            }
        }
        ColorStrategy::ColorTheory(hue) => {
            let filtered: Vec<&PatchSet> = area
                .iter()
                .filter(|p| {
                    p.theme_set
                        .iter()
                        .any(|a| matches!(a, ThemeSet::ColorTheory(_)))
                })
                .collect();
            let index = rng.gen_range(0..filtered.len());
            let selected = filtered[index];
            patch_single_area(level, patcher, rng, selected, hue, cfg);
        }
    }
}

fn patch_single_area(
    level: &str,
    patcher: &mut Patcher,
    rng: &mut ChaCha8Rng,
    selected: &PatchSet,
    hue_options: &HueOptions,
    cfg: &Config,
) {
    let flip = rng.gen_range(0..=1);
    let shift_distance = rng.gen_range(0..=11);
    let flip_allowed;
    let mode_all;
    match hue_options.flip_saturation {
        SaturationOptions::None => {
            flip_allowed = false;
            mode_all = false;
        }
        SaturationOptions::Safe => {
            flip_allowed = true;
            mode_all = false;
        }
        SaturationOptions::All => {
            flip_allowed = true;
            mode_all = true;
        }
    }
    let mut should_flip = false;
    if flip == 1 && flip_allowed && (mode_all || selected.saturation_flip_safe) {
        should_flip = true;
    }

    let colorshift = match hue_options.rotate_hue {
        true => format!("{}", shift_distance),
        false => "false".to_string(),
    };

    if cfg.log {
        println!(
            "picked {} theme for area {} shifted {} flip:{}",
            selected.name, level, colorshift, should_flip
        )
    }

    for values in &selected.patches {
        let mut shifted;
        match hue_options.rotate_hue {
            true => {
                shifted = shift_all_colors(values.hex_code, shift_distance);
            }
            false => shifted = values.hex_code.to_string(),
        }

        if should_flip {
            shifted = flip_all_saturation(&shifted);
        }

        patcher.add_change(&shifted, values.address);
    }
}

fn flip_all_saturation(colors: &str) -> String {
    let v1 = flip_single_saturation(&colors[..2]);
    let v2 = flip_single_saturation(&colors[2..4]);
    let v3 = flip_single_saturation(&colors[4..]);

    format!("{}{}{}", v1, v2, v3)
}
fn flip_single_saturation(color: &str) -> String {
    let mut chars = color.chars();
    let c1 = &chars.next().unwrap();
    let c2 = &chars.next().unwrap();
    let saturation = ['0', '1', '2', '3'];
    let hue = ['1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C'];
    if saturation.contains(c1) && hue.contains(c2) {
        match c1 {
            '0' => format!("{}{}", '3', c2),
            '1' => format!("{}{}", '2', c2),
            '2' => format!("{}{}", '1', c2),
            '3' => format!("{}{}", '0', c2),
            _ => color.to_string(),
        }
    } else {
        {
            color.to_string()
        }
    }
}
fn shift_all_colors(colors: &str, distance: usize) -> String {
    let v1 = shift_single_color_hue(&colors[..2], distance);
    let v2 = shift_single_color_hue(&colors[2..4], distance);
    let v3 = shift_single_color_hue(&colors[4..], distance);

    format!("{}{}{}", v1, v2, v3)
}
fn shift_single_color_hue(color: &str, distance: usize) -> String {
    let mut chars = color.chars();
    let c1 = &chars.next().unwrap();
    let c2 = &chars.next().unwrap();
    let saturation = ['0', '1', '2', '3'];
    let hue = ['1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C'];
    if saturation.contains(c1) && hue.contains(c2) {
        let index = hue.iter().position(|&c| c == *c2).unwrap();
        let new_index = (index + distance) % hue.len();

        format!("{}{}", c1, hue[new_index])
    } else {
        color.to_string()
    }
}

fn get_foreground_enemies() -> (&'static str, Vec<PatchSet>) {
    (
        "enemies",
        vec![PatchSet {
            name: "Vanilla",
            theme_set: vec![
                ThemeSet::All,
                ThemeSet::Vanilla,
                ThemeSet::ColorTheory(Triad),
            ],
            saturation_flip_safe: false,
            patches: vec![
                Patch {
                    // player and red white black
                    address: "1731A",
                    hex_code: "30160F",
                },
                Patch {
                    // white orange red
                    address: "1731D",
                    hex_code: "302706",
                },
                Patch {
                    //white blue
                    address: "17320",
                    hex_code: "31210F",
                },
                Patch {
                    //white green
                    address: "17323",
                    hex_code: "30290F",
                },
            ],
        }],
    )
}

fn get_final_boss_foreground() -> (&'static str, Vec<PatchSet>) {
    (
        "final",
        vec![PatchSet {
            name: "Vanilla",
            theme_set: vec![
                ThemeSet::All,
                ThemeSet::Vanilla,
                ThemeSet::ColorTheory(Complementary),
            ],
            saturation_flip_safe: false,
            patches: vec![
                Patch {
                    // player and red white black
                    address: "1737A",
                    hex_code: "301606",
                },
                Patch {
                    // white orange red
                    address: "1737D",
                    hex_code: "302717",
                },
                Patch {
                    //white blue
                    address: "17380",
                    hex_code: "2C1C0C",
                },
                Patch {
                    //white green
                    address: "17383",
                    hex_code: "3D1707",
                },
            ],
        }],
    )
}

fn get_foreground_title() -> (&'static str, Vec<PatchSet>) {
    (
        "title",
        vec![
            PatchSet {
                name: "Vanilla",
                theme_set: vec![ThemeSet::All, ThemeSet::Vanilla],
                saturation_flip_safe: false,
                patches: vec![
                    Patch {
                        // text
                        address: "17332",
                        hex_code: "302616",
                    },
                    Patch {
                        //stars
                        address: "17335",
                        hex_code: "301201",
                    },
                    Patch {
                        //planet inside (3 should match planet outside probably
                        address: "17338",
                        hex_code: "311611",
                    },
                    Patch {
                        //planet outside
                        address: "1733B",
                        hex_code: "312111",
                    },
                ],
            },
            PatchSet {
                name: "Rust",
                theme_set: vec![
                    ThemeSet::All,
                    ThemeSet::Crafted,
                    ThemeSet::ColorTheory(ColorTheory::Monochrome),
                ],
                saturation_flip_safe: false,
                patches: vec![
                    Patch {
                        // text
                        address: "17332",
                        hex_code: "071727",
                    },
                    Patch {
                        //stars and text
                        address: "17335",
                        hex_code: "383727",
                    },
                    Patch {
                        //planet inside (3 should match planet outside probably
                        address: "17338",
                        hex_code: "371707",
                    },
                    Patch {
                        //planet outside
                        address: "1733B",
                        hex_code: "271707",
                    },
                ],
            },
            PatchSet {
                name: "Asteroid",
                theme_set: vec![
                    ThemeSet::All,
                    ThemeSet::Crafted,
                    ThemeSet::ColorTheory(ColorTheory::Complementary),
                ],
                saturation_flip_safe: false,
                patches: vec![
                    Patch {
                        // text
                        address: "17332",
                        hex_code: "311101",
                    },
                    Patch {
                        //stars and text
                        address: "17335",
                        hex_code: "372717",
                    },
                    Patch {
                        //planet inside (3 should match planet outside probably
                        address: "17338",
                        hex_code: "312701",
                    },
                    Patch {
                        //planet outside
                        address: "1733B",
                        hex_code: "112101",
                    },
                ],
            },
            PatchSet {
                name: "Christmas Planet",
                theme_set: vec![
                    ThemeSet::All,
                    ThemeSet::Crafted,
                    ThemeSet::ColorTheory(ColorTheory::Triad),
                ],
                saturation_flip_safe: false,
                patches: vec![
                    Patch {
                        // text
                        address: "17332",
                        hex_code: "3A1A0A",
                    },
                    Patch {
                        //stars and text
                        address: "17335",
                        hex_code: "322202",
                    },
                    Patch {
                        //planet inside (3 should match planet outside probably
                        address: "17338",
                        hex_code: "3A2A06",
                    },
                    Patch {
                        //planet outside
                        address: "1733B",
                        hex_code: "261606",
                    },
                ],
            },
        ],
    )
}
fn get_c0() -> (&'static str, Vec<PatchSet>) {
    //patch c0
    // 0x17344 -> 1c,0c,16
    // used by background floor in c0
    // 0x1733E -> 11,01,16
    // used by foreground floor in c0
    // 0x17341 -> 22,12,16
    // used by eye cannons and stuff
    (
        "c0",
        vec![
            PatchSet {
                name: "Vanilla",
                theme_set: vec![ThemeSet::All, ThemeSet::Vanilla],
                saturation_flip_safe: false,
                patches: vec![
                    Patch {
                        address: "17344",
                        hex_code: "1C0C16",
                    },
                    Patch {
                        address: "1733E",
                        hex_code: "110116",
                    },
                    Patch {
                        address: "17341",
                        hex_code: "221216",
                    },
                ],
            },
            PatchSet {
                name: "Purple",
                theme_set: vec![ThemeSet::All, ThemeSet::Crafted],
                saturation_flip_safe: false,
                patches: vec![
                    Patch {
                        address: "17344",
                        hex_code: "140326",
                    },
                    Patch {
                        address: "1733E",
                        hex_code: "241326",
                    },
                    Patch {
                        address: "17341",
                        hex_code: "130307",
                    },
                ],
            },
            PatchSet {
                name: "Grayscale",
                theme_set: vec![ThemeSet::All, ThemeSet::Crafted],
                saturation_flip_safe: true,
                patches: vec![
                    Patch {
                        address: "17344",
                        hex_code: "3D2D27",
                    },
                    Patch {
                        address: "1733E",
                        hex_code: "3D2D27",
                    },
                    Patch {
                        address: "17341",
                        hex_code: "2D3D27",
                    },
                ],
            },
            PatchSet {
                name: "Icy",
                theme_set: vec![ThemeSet::All, ThemeSet::Crafted],
                saturation_flip_safe: false,
                patches: vec![
                    Patch {
                        address: "1733E",
                        hex_code: "3C2C32",
                    },
                    Patch {
                        address: "17341",
                        hex_code: "1C3C22",
                    },
                    Patch {
                        address: "17344",
                        hex_code: "1C0C31",
                    },
                ],
            },
            PatchSet {
                name: "Burnt",
                theme_set: vec![
                    ThemeSet::All,
                    ThemeSet::Crafted,
                    ThemeSet::ColorTheory(Monochrome),
                ],
                saturation_flip_safe: false,
                patches: vec![
                    Patch {
                        address: "1733E",
                        hex_code: "261707",
                    },
                    Patch {
                        address: "17341",
                        hex_code: "061628",
                    },
                    Patch {
                        address: "17344",
                        hex_code: "070801",
                    },
                ],
            },
            PatchSet {
                name: "GIJoe",
                theme_set: vec![
                    ThemeSet::All,
                    ThemeSet::Crafted,
                    ThemeSet::ColorTheory(ColorTheory::Triad),
                ],
                saturation_flip_safe: false,
                patches: vec![
                    Patch {
                        address: "1733E",
                        hex_code: "250521",
                    },
                    Patch {
                        address: "17341",
                        hex_code: "120215",
                    },
                    Patch {
                        address: "17344",
                        hex_code: "180835",
                    },
                ],
            },
            PatchSet {
                name: "AlmostNormal",
                theme_set: vec![
                    ThemeSet::All,
                    ThemeSet::Crafted,
                    ThemeSet::ColorTheory(ColorTheory::Complementary),
                ],
                saturation_flip_safe: false,
                patches: vec![
                    Patch {
                        address: "1733E",
                        hex_code: "120129",
                    },
                    Patch {
                        address: "17341",
                        hex_code: "021216",
                    },
                    Patch {
                        address: "17344",
                        hex_code: "160819",
                    },
                ],
            },
            PatchSet {
                name: "Shadow",
                theme_set: vec![ThemeSet::All, ThemeSet::Crafted],
                saturation_flip_safe: false,
                patches: vec![
                    Patch {
                        address: "1733E",
                        hex_code: "010D38",
                    },
                    Patch {
                        address: "17341",
                        hex_code: "010D38",
                    },
                    Patch {
                        address: "17344",
                        hex_code: "0D0D08",
                    },
                ],
            },
        ],
    )
}

fn get_a0() -> (&'static str, Vec<PatchSet>) {
    (
        "a0",
        vec![
            PatchSet {
                name: "Vanilla",
                theme_set: vec![ThemeSet::All, ThemeSet::Vanilla],
                saturation_flip_safe: true,
                patches: vec![
                    Patch {
                        address: "17326",
                        hex_code: "301A0A",
                    },
                    Patch {
                        address: "1732F",
                        hex_code: "301000",
                    },
                ],
            },
            PatchSet {
                name: "Sand",
                theme_set: vec![
                    ThemeSet::All,
                    ThemeSet::Crafted,
                    ThemeSet::ColorTheory(Monochrome),
                ],
                saturation_flip_safe: true,
                patches: vec![
                    Patch {
                        address: "17326",
                        hex_code: "362838",
                    },
                    Patch {
                        address: "1732F",
                        hex_code: "362717",
                    },
                ],
            },
            PatchSet {
                name: "Barbie",
                theme_set: vec![ThemeSet::All, ThemeSet::Crafted],
                saturation_flip_safe: true,
                patches: vec![
                    Patch {
                        address: "17326",
                        hex_code: "241504",
                    },
                    Patch {
                        address: "1732F",
                        hex_code: "243424",
                    },
                ],
            },
            PatchSet {
                name: "Dank",
                theme_set: vec![ThemeSet::All, ThemeSet::Crafted],
                saturation_flip_safe: false,
                patches: vec![
                    Patch {
                        address: "17326",
                        hex_code: "0A1D08",
                    },
                    Patch {
                        address: "1732F",
                        hex_code: "0A0B0C",
                    },
                ],
            },
            PatchSet {
                name: "Grandmas",
                theme_set: vec![
                    ThemeSet::All,
                    ThemeSet::Crafted,
                    ThemeSet::ColorTheory(Triad),
                ],
                saturation_flip_safe: true,
                patches: vec![
                    Patch {
                        address: "17326",
                        hex_code: "3A0717",
                    },
                    Patch {
                        address: "1732F",
                        hex_code: "3A1A03",
                    },
                ],
            },
            PatchSet {
                name: "Golf",
                theme_set: vec![
                    ThemeSet::All,
                    ThemeSet::Crafted,
                    ThemeSet::ColorTheory(Complementary),
                ],
                saturation_flip_safe: true,
                patches: vec![
                    Patch {
                        address: "17326",
                        hex_code: "390312",
                    },
                    Patch {
                        address: "1732F",
                        hex_code: "392719",
                    },
                ],
            },
        ],
    )
}

fn get_a1() -> (&'static str, Vec<PatchSet>) {
    (
        "a1",
        // good test level is 11/OB as it has both grass and the firing enemies
        vec![
            //vanilla
            PatchSet {
                name: "Vanilla",
                theme_set: vec![ThemeSet::All, ThemeSet::Vanilla],
                saturation_flip_safe: false,
                patches: vec![
                    //1. used for grass, middle almost certainly must match
                    Patch {
                        address: "17365",
                        hex_code: "2A1C1A",
                    },
                    //2. firing ground plants, middle color should match ground middle
                    //last color is used for the center of the bubble spawners
                    Patch {
                        address: "17368",
                        hex_code: "301C16",
                    },
                    //3. ground colors, middle should match 2 middle
                    Patch {
                        address: "1736B",
                        hex_code: "2C1C0C",
                    },
                ],
            },
            PatchSet {
                name: "Deep Blue",
                theme_set: vec![ThemeSet::All, ThemeSet::Crafted],
                saturation_flip_safe: false,
                patches: vec![
                    //1. used for grass, middle almost certainly must match
                    Patch {
                        address: "17365",
                        hex_code: "180104",
                    },
                    //2. firing ground plants, middle color should match ground middle
                    //last color is used for the center of the bubble spawners
                    Patch {
                        address: "17368",
                        hex_code: "140127",
                    },
                    //3. ground colors, middle should match 2 middle
                    Patch {
                        address: "1736B",
                        hex_code: "110102",
                    },
                ],
            },
            PatchSet {
                //this theme kinda looks like hot garbagio but it shifts mostly fine, consider for replacement
                name: "Algae",
                theme_set: vec![
                    ThemeSet::All,
                    ThemeSet::Crafted,
                    ThemeSet::ColorTheory(ColorTheory::Monochrome),
                ],
                saturation_flip_safe: false,
                patches: vec![
                    //1. used for grass, middle almost certainly must match
                    Patch {
                        address: "17365",
                        hex_code: "1B0A1A",
                    },
                    //2. firing ground plants, middle color should match ground middle
                    //last color is used for the center of the bubble spawners
                    Patch {
                        address: "17368",
                        hex_code: "1A0A29",
                    },
                    //3. ground colors, middle should match 2 middle
                    Patch {
                        address: "1736B",
                        hex_code: "090A0B",
                    },
                ],
            },
            PatchSet {
                //this theme kinda looks like hot garbagio but it shifts mostly fine, consider for replacement
                name: "Pink Slime",
                theme_set: vec![
                    ThemeSet::All,
                    ThemeSet::Crafted,
                    ThemeSet::ColorTheory(ColorTheory::Complementary),
                ],
                saturation_flip_safe: false,
                patches: vec![
                    //1. used for grass, middle almost certainly must match
                    Patch {
                        address: "17365",
                        hex_code: "190408",
                    },
                    //2. firing ground plants, middle color should match ground middle
                    //last color is used for the center of the bubble spawners
                    Patch {
                        address: "17368",
                        hex_code: "180429",
                    },
                    //3. ground colors, middle should match 2 middle
                    Patch {
                        address: "1736B",
                        hex_code: "140403",
                    },
                ],
            },
            PatchSet {
                //this theme kinda looks like hot garbagio but it shifts mostly fine, consider for replacement
                name: "Pale Blue",
                theme_set: vec![
                    ThemeSet::All,
                    ThemeSet::Crafted,
                    ThemeSet::ColorTheory(ColorTheory::Triad),
                ],
                saturation_flip_safe: false,
                patches: vec![
                    //1. used for grass, middle almost certainly must match
                    Patch {
                        address: "17365",
                        hex_code: "292218",
                    },
                    //2. firing ground plants, middle color should match ground middle
                    //last color is used for the center of the bubble spawners
                    Patch {
                        address: "17368",
                        hex_code: "252205",
                    },
                    //3. ground colors, middle should match 2 middle
                    Patch {
                        address: "1736B",
                        hex_code: "112212",
                    },
                ],
            },
        ],
    )
}

fn get_a3() -> (&'static str, Vec<PatchSet>) {
    (
        "a3", //test on c14/OE
        vec![
            PatchSet {
                name: "Vanilla",
                theme_set: vec![ThemeSet::All, ThemeSet::Vanilla],
                saturation_flip_safe: false,
                patches: vec![
                    //1. used for flowers,
                    // first 2 colors should match 2
                    // Final value should match final value of 3
                    Patch {
                        address: "1735C",
                        hex_code: "30160A",
                    },
                    //2. demon plant,
                    // first 2 colors should match 1
                    // last color should be close
                    Patch {
                        address: "1735F",
                        hex_code: "301606",
                    },
                    //3. background
                    Patch {
                        address: "17362",
                        hex_code: "2A1A0A",
                    },
                ],
            },
            PatchSet {
                name: "Fall",
                theme_set: vec![
                    ThemeSet::All,
                    ThemeSet::Crafted,
                    ThemeSet::ColorTheory(ColorTheory::Monochrome),
                ],
                saturation_flip_safe: false,
                patches: vec![
                    //1. used for flowers,
                    // first 2 colors should match 2
                    // Final value should match final value of 3
                    Patch {
                        address: "1735C",
                        hex_code: "281707",
                    },
                    //2. demon plant,
                    // first 2 colors should match 1
                    // last color should be close
                    Patch {
                        address: "1735F",
                        hex_code: "271706",
                    },
                    //3. background
                    Patch {
                        address: "17362",
                        hex_code: "261707",
                    },
                ],
            },
            PatchSet {
                name: "Musk",
                theme_set: vec![
                    ThemeSet::All,
                    ThemeSet::Crafted,
                    ThemeSet::ColorTheory(ColorTheory::Complementary),
                ],
                saturation_flip_safe: false,
                patches: vec![
                    //1. used for flowers,
                    // first 2 colors should match 2
                    // Final value should match final value of 3
                    Patch {
                        address: "1735C",
                        hex_code: "240308",
                    },
                    //2. demon plant,
                    // first 2 colors should match 1
                    // last color should be close
                    Patch {
                        address: "1735F",
                        hex_code: "241303",
                    },
                    //3. background
                    Patch {
                        address: "17362",
                        hex_code: "381808",
                    },
                ],
            },
            PatchSet {
                name: "Soil",
                theme_set: vec![
                    ThemeSet::All,
                    ThemeSet::Crafted,
                    ThemeSet::ColorTheory(ColorTheory::Triad),
                ],
                saturation_flip_safe: false,
                patches: vec![
                    //1. used for flowers,
                    // first 2 colors should match 2
                    // Final value should match final value of 3
                    Patch {
                        address: "1735C",
                        hex_code: "230307",
                    },
                    //2. demon plant,
                    // first 2 colors should match 1
                    // last color should be close
                    Patch {
                        address: "1735F",
                        hex_code: "230313",
                    },
                    //3. background
                    Patch {
                        address: "17362",
                        hex_code: "1b0b07",
                    },
                ],
            },
        ],
    )
}

fn get_a7() -> (&'static str, Vec<PatchSet>) {
    (
        "a7",
        vec![
            PatchSet {
                name: "Vanilla",
                theme_set: vec![ThemeSet::All, ThemeSet::Vanilla],
                saturation_flip_safe: false,
                patches: vec![
                    //1. eyeballs seem to be it, last digit should match last digit of 03
                    Patch {
                        address: "17353",
                        hex_code: "302112",
                    },
                    //2. i think this is the blue plants
                    Patch {
                        address: "17356",
                        hex_code: "221205",
                    },
                    //3. background
                    Patch {
                        address: "17359",
                        hex_code: "250605",
                    },
                ],
            },
            PatchSet {
                name: "Blue Blood",
                theme_set: vec![
                    ThemeSet::All,
                    ThemeSet::Crafted,
                    ThemeSet::ColorTheory(ColorTheory::Monochrome),
                ],
                saturation_flip_safe: false,
                patches: vec![
                    //1. eyeballs seem to be it, last digit should match last digit of 03
                    Patch {
                        address: "17353",
                        hex_code: "303121",
                    },
                    //2. i think this is the blue plants
                    Patch {
                        address: "17356",
                        hex_code: "312102",
                    },
                    //3. background
                    Patch {
                        address: "17359",
                        hex_code: "012102",
                    },
                ],
            },
            PatchSet {
                name: "Sickly",
                theme_set: vec![
                    ThemeSet::All,
                    ThemeSet::Crafted,
                    ThemeSet::ColorTheory(ColorTheory::Triad),
                ],
                saturation_flip_safe: false,
                patches: vec![
                    //1. eyeballs seem to be it, last digit should match last digit of 03
                    Patch {
                        address: "17353",
                        hex_code: "311101",
                    },
                    //2. i think this is the blue plants
                    Patch {
                        address: "17356",
                        hex_code: "280805",
                    },
                    //3. background
                    Patch {
                        address: "17359",
                        hex_code: "281505",
                    },
                ],
            },
            PatchSet {
                name: "Flesh",
                theme_set: vec![
                    ThemeSet::All,
                    ThemeSet::Crafted,
                    ThemeSet::ColorTheory(ColorTheory::Complementary),
                ],
                saturation_flip_safe: false,
                patches: vec![
                    //1. eyeballs seem to be it, last digit should match last digit of 03
                    Patch {
                        address: "17353",
                        hex_code: "332203",
                    },
                    //2. i think this is the blue plants
                    Patch {
                        address: "17356",
                        hex_code: "120308",
                    },
                    //3. background
                    Patch {
                        address: "17359",
                        hex_code: "182808",
                    },
                ],
            },
        ],
    )
}

fn get_a9() -> (&'static str, Vec<PatchSet>) {
    (
        "a9",
        vec![
            PatchSet {
                name: "Vanilla",
                theme_set: vec![ThemeSet::All, ThemeSet::Vanilla],
                saturation_flip_safe: false,
                patches: vec![
                    //1. seems nothing
                    //Patch{address: "1734A", hex_code: "271707"},
                    //2. seems nothing
                    //Patch{address: "1734D", hex_code: "37170F"},
                    //3. background
                    Patch {
                        address: "17350",
                        hex_code: "281808",
                    },
                ],
            },
            PatchSet {
                name: "Pepto",
                theme_set: vec![
                    ThemeSet::All,
                    ThemeSet::Crafted,
                    ThemeSet::ColorTheory(Monochrome),
                ],
                saturation_flip_safe: false,
                patches: vec![
                    //1. seems nothing
                    //Patch{address: "1734A", hex_code: "271707"},
                    //2. seems nothing
                    //Patch{address: "1734D", hex_code: "37170F"},
                    //3. background
                    Patch {
                        address: "17350",
                        hex_code: "342515",
                    },
                ],
            },
            PatchSet {
                name: "Shadow",
                theme_set: vec![
                    ThemeSet::All,
                    ThemeSet::Crafted,
                    ThemeSet::ColorTheory(Complementary),
                ],
                saturation_flip_safe: false,
                patches: vec![
                    //1. seems nothing
                    //Patch{address: "1734A", hex_code: "271707"},
                    //2. seems nothing
                    //Patch{address: "1734D", hex_code: "37170F"},
                    //3. background
                    Patch {
                        address: "17350",
                        hex_code: "170701",
                    },
                ],
            },
            PatchSet {
                name: "Spacey",
                theme_set: vec![
                    ThemeSet::All,
                    ThemeSet::Crafted,
                    ThemeSet::ColorTheory(Triad),
                ],
                saturation_flip_safe: false,
                patches: vec![
                    //1. seems nothing
                    //Patch{address: "1734A", hex_code: "271707"},
                    //2. seems nothing
                    //Patch{address: "1734D", hex_code: "37170F"},
                    //3. background
                    Patch {
                        address: "17350",
                        hex_code: "392105",
                    },
                ],
            },
        ],
    )
}

fn get_a5() -> (&'static str, Vec<PatchSet>) {
    (
        "a5", //test on 16, 0F
        vec![
            PatchSet {
                name: "Vanilla",
                theme_set: vec![ThemeSet::All, ThemeSet::Vanilla],
                saturation_flip_safe: false,
                patches: vec![
                    //1. blue orb background
                    Patch {
                        address: "1736E",
                        hex_code: "311202",
                    },
                    //2. volcano center, also it seems like save room tiles
                    Patch {
                        address: "17371",
                        hex_code: "351505",
                    },
                    //3. background
                    Patch {
                        address: "17374",
                        hex_code: "301000",
                    },
                    // the floor hexes in the overworld are shared with
                    // the crates for some reason, we still load volcano color
                    // does something use that?
                ],
            },
            PatchSet {
                name: "Sunset",
                theme_set: vec![
                    ThemeSet::All,
                    ThemeSet::Crafted,
                    ThemeSet::ColorTheory(Monochrome),
                ],
                saturation_flip_safe: false,
                patches: vec![
                    Patch {
                        address: "1736E",
                        hex_code: "381808",
                    },
                    Patch {
                        address: "17371",
                        hex_code: "281808",
                    },
                    Patch {
                        address: "17374",
                        hex_code: "373828",
                    },
                ],
            },
            PatchSet {
                name: "Uranus",
                theme_set: vec![
                    ThemeSet::All,
                    ThemeSet::Crafted,
                    ThemeSet::ColorTheory(Complementary),
                ],
                saturation_flip_safe: false,
                patches: vec![
                    Patch {
                        address: "1736E",
                        hex_code: "241505",
                    },
                    Patch {
                        address: "17371",
                        hex_code: "251505",
                    },
                    Patch {
                        address: "17374",
                        hex_code: "2B3A1B",
                    },
                ],
            },
            PatchSet {
                name: "RGB",
                theme_set: vec![
                    ThemeSet::All,
                    ThemeSet::Crafted,
                    ThemeSet::ColorTheory(Triad),
                ],
                saturation_flip_safe: false,
                patches: vec![
                    Patch {
                        address: "1736E",
                        hex_code: "3A2A0A",
                    },
                    Patch {
                        address: "17371",
                        hex_code: "261606",
                    },
                    Patch {
                        address: "17374",
                        hex_code: "221202",
                    },
                ],
            },
        ],
    )
}

fn move_a5_floor_color_to_volcano(patcher: &mut Patcher) {
    //0x17e44 -> "02" will move the palette from 01 to 02 in area 5s floor, replacing the 4 things before it would replace that tile
    //patcher.add_change("02","17e44");

    //need three things
    //first need to set the ground to some completely garbage tile i saw in the rom
    patcher.add_change("FCFCFCFC02", "17F58");

    //then i need to set the actual ground to that tile
    patcher.add_change("8E", "16C1B");

    //then i need to patch the area 5 loading to grab the starfield color instead of the volcano color
    patcher.add_change("1C", "16EA4");

    //patch the saveroom too since it hits volcano
    patcher.add_change("0407", "16EB3");
}

fn get_ran_palette(rng: &mut ChaCha8Rng) -> String {
    let colors = vec![
        "01", "02", "03", "04", "05", "06", "07", "08", "09", "0A", "0B", "0C", "11", "12", "13",
        "14", "15", "16", "17", "18", "19", "1A", "1B", "1C", "21", "22", "23", "24", "25", "26",
        "27", "28", "29", "2A", "2B", "2C", "31", "32", "33", "34", "35", "36", "37", "38", "39",
        "3A", "3B", "3C", "1D", "2D", "3D", "30",
    ];
    let c1 = rng.gen_range(0..colors.len());
    let c2 = rng.gen_range(0..colors.len());
    let c3 = rng.gen_range(0..colors.len());

    format!("{}{}{}", c1, c2, c3)
}
