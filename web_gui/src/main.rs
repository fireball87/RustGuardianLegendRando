mod gui_error;

use crate::gui_error::GuiError;
use dioxus::prelude::*;
use tgl_rando_core::config::*;
use tgl_rando_core::patcher::Patcher;
use tgl_rando_core::tgl_error::TGLError;
use tgl_rando_core::{generate, seed};

fn main() {
    // launch the web app
    launch(app);
}

// create a component that renders a div with the text "Hello, world!"
fn app() -> Element {
    //let response = setup();
    //let mut filenames: Signal<Vec<String>> = use_signal(Vec::new);
    //let mut files_uploaded: Signal<Vec<String>> = use_signal(Vec::new);
    //let mut uploaded: Signal<Vec<u8>> = use_signal(Vec::new);
    let mut file_name: Signal<String> = use_signal(String::new);
    let mut uploaded: Signal<Vec<u8>> = use_signal(Vec::new);

    //let defaultCfg = build_default_cfg();
    // make a signal even though it doesn't need to be?
    let corridor_cfg = use_signal(CorridorConfig::default);
    let boss_cfg = use_signal(BossConfig::default);
    let qol_cfg = use_signal(QOLHacks::default);
    let color_cfg = use_signal(ColorOptions::default);
    let hue_cfg = use_signal(HueOptions::default);

    let mut advanced_visible = use_signal(|| false);
    let map_cfg = use_signal(MapConfig::default);
    let item_cfg = use_signal(ItemConfig::default);

    let mut err_string = use_signal(String::new);

    rsx! {
        div {
            corridor_config { c: corridor_cfg }
            boss_config { c: boss_cfg }
            qol_hacks { c: qol_cfg }
            color_config { c: color_cfg, h: hue_cfg }

            h3 { "Advanced options." }
            input {
                r#type: "checkbox",
                checked: advanced_visible(),
                id: "advanced_options",
                oninput: move |event| { advanced_visible.set(event.value().parse().unwrap_or(false)) }
            }
            label { "for": "advanced_options",
                "Show Advanced Options, not checked for sanity so may produce errors or perhaps broken roms."
            }
            br {}

            if advanced_visible() {
                map_config { c: map_cfg }
                item_config { c: item_cfg }
            }

            h3 { "Generate" }

            if !err_string().is_empty() {
                h4 { color: "Red", "Error: {err_string}" }
            }
            input {
                // tell the input to pick a file
                r#type: "file",
                // list the accepted extensions
                accept: ".rom,.nes,.bin",
                // pick multiple files
                multiple: false,
                id: "romSelect",

                onchange: move |evt| {
                    async move {
                        if let Some(file_engine) = &evt.files() {
                            let files = file_engine.files();
                            for name in files {
                                if let Some(file) = file_engine.read_file(&name).await {
                                    uploaded.replace(file);
                                }
                                file_name.replace(name);
                            }
                        }
                    }
                }
            }
            label { "for": "romSelect", "Select Base Rom" }
            br {}
            button {
                disabled: uploaded().is_empty(),
                onclick: move |_| {
                    let colors = match color_cfg().color_strategy {
                        ColorStrategy::Vanilla(_) => ColorStrategy::Vanilla(hue_cfg()),
                        ColorStrategy::All(_) => ColorStrategy::All(hue_cfg()),
                        ColorStrategy::Random => ColorStrategy::Random,
                        ColorStrategy::ColorTheory(_) => ColorStrategy::ColorTheory(hue_cfg()),
                    };
                    match patch_file(
                        "tgl_rando.nes",
                        &(uploaded.read()),
                        Config {
                            corridor_config: corridor_cfg(),
                            map_config: map_cfg(),
                            item_config: item_cfg(),
                            qol_hacks: qol_cfg(),
                            color_options: ColorOptions {
                                color_strategy: colors,
                                ..color_cfg()
                            },
                            boss_config: boss_cfg(),
                            log: false,
                            seed: seed::make_seed(),
                        },
                        err_string,
                    ) {
                        Ok(_) => {
                            err_string.set("".to_string());
                        }
                        Err(e) => {
                            err_string.set(e.message);
                        }
                    }
                },
                "Generate"
            }
            br {}
            br {}
        }
    }
}

//pub shuffle_skies: bool,
//pub shuffle_ground: bool,
//pub shuffle_corridors: bool,
#[component]
fn corridor_config(c: Signal<CorridorConfig>) -> Element {
    rsx! {
        div {
            h3 { "Corridor Config" }
            input {
                r#type: "checkbox",
                checked: c().shuffle_corridors,
                id: "shuffleCorridors",
                oninput: move |event| {
                    c.set(CorridorConfig {
                        shuffle_corridors: event.value().parse().unwrap_or(false),
                        ..c()
                    })
                }
            }
            label { "for": "shuffleCorridors", "Shuffle Corridors" }
            br {}

            input {
                r#type: "checkbox",
                checked: c().shuffle_ground,
                id: "shuffle_ground",
                oninput: move |event| {
                    c.set(CorridorConfig {
                        shuffle_ground: event.value().parse().unwrap_or(false),
                        ..c()
                    })
                }
            }
            label { "for": "shuffleGround", "Shuffle Ground !!Needs Testing!!" }
            br {}

            input {
                r#type: "checkbox",
                checked: c().shuffle_skies,
                id: "shuffle_skies",
                oninput: move |event| {
                    c.set(CorridorConfig {
                        shuffle_skies: event.value().parse().unwrap_or(false),
                        ..c()
                    })
                }
            }
            label { "for": "shuffleSkies", "Shuffle Air Enemy Spawns" }
            br {}
        }
    }
}

//pub shuffle_bosses: bool,
//pub shuffle_final_boss: bool,
//pub rebalance_bosses: bool,
//pub randomize_boss_health: bool,
#[component]
fn boss_config(c: Signal<BossConfig>) -> Element {
    rsx! {
        div {
            h3 { "Boss Config" }

            input {
                r#type: "checkbox",
                checked: c().shuffle_bosses,
                id: "shuffle_bosses",
                oninput: move |event| {
                    c.set(BossConfig {
                        shuffle_bosses: event.value().parse().unwrap_or(false),
                        ..c()
                    })
                }
            }
            label { "for": "shuffle_bosses", "Shuffle Bosses" }
            br {}

            input {
                r#type: "checkbox",
                checked: c().shuffle_final_boss,
                id: "shuffle_final_boss",
                oninput: move |event| {
                    c.set(BossConfig {
                        shuffle_final_boss: event.value().parse().unwrap_or(false),
                        ..c()
                    })
                }
            }
            label { "for": "shuffle_final_boss", "Shuffle Final Boss (if shuffling bosses)" }
            br {}

            input {
                r#type: "checkbox",
                checked: c().rebalance_bosses,
                id: "rebalance_bosses",
                oninput: move |event| {
                    c.set(BossConfig {
                        rebalance_bosses: event.value().parse().unwrap_or(false),
                        ..c()
                    })
                }
            }
            label { "for": "rebalance_bosses",
                "Rebalance Bosses and apply Scaling Hack (will act as if true if either Shuffle Corridor or Shuffle Bosses is true.)"
            }
            br {}

            input {
                r#type: "checkbox",
                checked: c().randomize_boss_health,
                id: "randomize_boss_health",
                oninput: move |event| {
                    c.set(BossConfig {
                        randomize_boss_health: event.value().parse().unwrap_or(false),
                        ..c()
                    })
                }
            }
            label { "for": "randomize_boss_health",
                "Randomize Boss Health Values (if we are rebalancing them)"
            }
            br {}
        }
    }
}

/*faster_starting_fire: true,
fix_hyper_laser: true,
enemy_erasers_unlocked_from_start: true,
remove_flash: true,*/
#[component]
fn qol_hacks(c: Signal<QOLHacks>) -> Element {
    rsx! {
        div {
            h3 { "Quality Of Life Hacks" }

            input {
                r#type: "checkbox",
                checked: c().faster_starting_fire,
                id: "faster_starting_fire",
                oninput: move |event| {
                    c.set(QOLHacks {
                        faster_starting_fire: event.value().parse().unwrap_or(false),
                        ..c()
                    })
                }
            }
            label { "for": "faster_starting_fire", "Increase Starting Fire Rate" }
            br {}

            input {
                r#type: "checkbox",
                checked: c().fix_hyper_laser,
                id: "fix_hyper_laser",
                oninput: move |event| {
                    c.set(QOLHacks {
                        fix_hyper_laser: event.value().parse().unwrap_or(false),
                        ..c()
                    })
                }
            }
            label { "for": "fix_hyper_laser", "Buff Hyperlaser Damage" }
            br {}

            input {
                r#type: "checkbox",
                checked: c().enemy_erasers_unlocked_from_start,
                id: "enemy_erasers_unlocked_from_start",
                oninput: move |event| {
                    c.set(QOLHacks {
                        enemy_erasers_unlocked_from_start: event.value().parse().unwrap_or(false),
                        ..c()
                    })
                }
            }
            label { "for": "enemy_erasers_unlocked_from_start", "Unlock Enemy Eraser Drops At Game Start" }
            br {}

            input {
                r#type: "checkbox",
                checked: c().remove_flash,
                id: "remove_flash",
                oninput: move |event| {
                    c.set(QOLHacks {
                        remove_flash: event.value().parse().unwrap_or(false),
                        ..c()
                    })
                }
            }
            label { "for": "remove_flash", "Remove Flashing (enemy erasers and boss kills)" }
            br {}
        }
    }
}

#[component]
fn color_config(c: Signal<ColorOptions>, h: Signal<HueOptions>) -> Element {
    rsx! {
        div {
            h3 { "Color Config" }
        }
        select {
            id: "theme",
            oninput: move |event| {
                match &*event.value() {
                    "All" => {
                        c.set(ColorOptions {
                            color_strategy: ColorStrategy::All(h()),
                            ..c()
                        })
                    }
                    "ColorTheory" => {
                        c.set(ColorOptions {
                            color_strategy: ColorStrategy::ColorTheory(h()),
                            ..c()
                        })
                    }
                    "Vanilla" => {
                        c.set(ColorOptions {
                            color_strategy: ColorStrategy::Vanilla(h()),
                            ..c()
                        })
                    }
                    "Random" => {
                        c.set(ColorOptions {
                            color_strategy: ColorStrategy::Random,
                            ..c()
                        })
                    }
                    _ => {}
                }
            },
            optgroup { label: "Themed",
                option { value: "All", label: "All", selected: matches!(c().color_strategy, ColorStrategy::All(_)) }
                option { value: "ColorTheory", label: "ColorTheory", selected: matches!(c().color_strategy, ColorStrategy::ColorTheory(_)) }
                option { value: "Vanilla", label: "Vanilla", selected: matches!(c().color_strategy, ColorStrategy::Vanilla(_)) }
            }
            optgroup { label: "Eye Bleed",
                option { value: "Random", label: "Random", selected: matches!(c().color_strategy, ColorStrategy::Random) }
            }
        }
        label { "for": "theme", "Recolor Mode" }
        br {}
        hue_config { c, h }
        input {
            r#type: "checkbox",
            checked: c().include_foreground,
            id: "include_foreground",
            oninput: move |event| {
                c.set(ColorOptions {
                    include_foreground: event.value().parse().unwrap_or(false),
                    ..c()
                })
            }
        }
        label { "for": "include_foreground", "Recolor Foreground Objects Too" }
        br {}
    }
}
#[component]
fn hue_config(c: Signal<ColorOptions>, h: Signal<HueOptions>) -> Element {
    rsx! {
        input {
            r#type: "checkbox",
            checked: h().rotate_hue,
            id: "rotate_hue",
            disabled: c().color_strategy == ColorStrategy::Random,
            oninput: move |event| {
                h.set(HueOptions {
                    rotate_hue: event.value().parse().unwrap_or(false),
                    ..h()
                })
            }
        }
        label { "for": "rotate_hue", "Rotate Hue" }
        br {}
        select {
            disabled: c().color_strategy == ColorStrategy::Random,
            id: "saturation",
            oninput: move |event| {
                match &*event.value() {
                    "None" => {
                        h.set(HueOptions {
                            flip_saturation: SaturationOptions::None,
                            ..h()
                        })
                    }
                    "Safe" => {
                        h.set(HueOptions {
                            flip_saturation: SaturationOptions::Safe,
                            ..h()
                        })
                    }
                    "All" => {
                        h.set(HueOptions {
                            flip_saturation: SaturationOptions::All,
                            ..h()
                        })
                    }
                    _ => {}
                }
            },
            option { value: "None", label: "None", selected: h().flip_saturation == SaturationOptions::None }
            option { value: "Safe", label: "Safe", selected: h().flip_saturation == SaturationOptions::Safe }
            option { value: "All", label: "All", selected: h().flip_saturation == SaturationOptions::All }
        }
        label { "for": "theme", "Saturation Flip Mode" }
        br {}
    }
}

// weapon_size: 4,
// blue: 9,
// red: 10,
// shield: 6,
// force_shields: true,
// guns: 5,
// rapid_fires: 5,
// etanks: 3,
// enemy_erasers: 5,
#[component]
fn item_config(c: Signal<ItemConfig>) -> Element {
    rsx! {
        div {
            h3 { "Item Config" }
            input {
                r#type: "range",
                min: 0,
                max: 6,
                value: c().weapon_size as i64,
                oninput: move |event| {
                    c.set(ItemConfig {
                        weapon_size: event.value().parse().unwrap_or(c().weapon_size),
                        ..c()
                    })
                },
                id: "weapon_size"
            }
            label { "for": "weapon_size", "{c().weapon_size} - Forced copies of each weapon" }
            br {}

            input {
                r#type: "range",
                min: 0,
                max: 20,
                value: c().blue as i64,
                oninput: move |event| {
                    c.set(ItemConfig {
                        blue: event.value().parse().unwrap_or(c().blue),
                        ..c()
                    })
                },
                id: "blue"
            }
            label { "for": "blue", "{c().blue} - Blue Landers" }
            br {}

            input {
                r#type: "range",
                min: 0,
                max: 20,
                value: c().red as i64,
                oninput: move |event| {
                    c.set(ItemConfig {
                        red: event.value().parse().unwrap_or(c().red),
                        ..c()
                    })
                },
                id: "red"
            }
            label { "for": "red", "{c().red} - Red Landers" }
            br {}

            input {
                r#type: "range",
                min: 0,
                max: 20,
                value: c().shield as i64,
                oninput: move |event| {
                    c.set(ItemConfig {
                        shield: event.value().parse().unwrap_or(c().shield),
                        ..c()
                    })
                },
                id: "shield"
            }
            label { "for": "shield", "{c().shield} - Shields" }
            br {}

            input {
                r#type: "checkbox",
                checked: c().force_shields,
                id: "force_shields",
                oninput: move |event| {
                    c.set(ItemConfig {
                        force_shields: event.value().parse().unwrap_or(false),
                        ..c()
                    })
                }
            }
            label { "for": "force_shields", "Force distribute 1 shield to every other area." }
            br {}

            input {
                r#type: "range",
                min: 0,
                max: 20,
                value: c().guns as i64,
                oninput: move |event| {
                    c.set(ItemConfig {
                        guns: event.value().parse().unwrap_or(c().guns),
                        ..c()
                    })
                },
                id: "guns"
            }
            label { "for": "guns", "{c().guns} - Gun Powerups" }
            br {}

            input {
                r#type: "range",
                min: 0,
                max: 20,
                value: c().rapid_fires as i64,
                oninput: move |event| {
                    c.set(ItemConfig {
                        rapid_fires: event.value().parse().unwrap_or(c().rapid_fires),
                        ..c()
                    })
                },
                id: "rapid_fires"
            }
            label { "for": "rapid_fires", "{c().rapid_fires} - Rapid Fires" }

            br {}
            input {
                r#type: "range",
                min: 0,
                max: 20,
                value: c().etanks as i64,
                oninput: move |event| {
                    c.set(ItemConfig {
                        etanks: event.value().parse().unwrap_or(c().etanks),
                        ..c()
                    })
                },
                id: "etanks"
            }
            label { "for": "etanks", "{c().etanks} - Energy Tanks" }
            br {}

            input {
                r#type: "range",
                min: 0,
                max: 20,
                value: c().enemy_erasers as i64,
                oninput: move |event| {
                    c.set(ItemConfig {
                        enemy_erasers: event.value().parse().unwrap_or(c().enemy_erasers),
                        ..c()
                    })
                },
                id: "enemy_erasers"
            }
            label { "for": "enemy_erasers", "{c().enemy_erasers} - Enemy Erasers" }
            br {}
        }
    }
}

//currently excluded are the two shop sizes
#[component]
fn map_config(c: Signal<MapConfig>) -> Element {
    rsx! {
        div {
            h3 { "Labyrinth Config" }
            input {
                r#type: "range",
                min: 10,
                max: c().max_area_size as i64,
                value: c().min_area_size as i64,
                oninput: move |event| {
                    c.set(MapConfig {
                        min_area_size: event.value().parse().unwrap_or(c().min_area_size),
                        ..c()
                    })
                },
                id: "min_area_size"
            }
            label { "for": "min_area_size", "{c().min_area_size} - Minimum Area Size" }
            br {}

            input {
                r#type: "range",
                min: c().min_area_size as i64,
                max: 30,
                value: c().max_area_size as i64,
                oninput: move |event| {
                    c.set(MapConfig {
                        max_area_size: event.value().parse().unwrap_or(c().max_area_size),
                        ..c()
                    })
                },
                id: "max_area_size"
            }
            label { "for": "max_area_size", "{c().max_area_size} - Maximum Area Size" }
            br {}

            input {
                r#type: "range",
                min: 0,
                max: 10,
                value: c().desired_connections as i64,
                oninput: move |event| {
                    c.set(MapConfig {
                        desired_connections: event
                            .value()
                            .parse()
                            .unwrap_or(c().desired_connections),
                        ..c()
                    })
                },
                id: "desired_connections"
            }
            label { "for": "desired_connections", "{c().desired_connections} - Desired Connections" }
            br {}

            input {
                r#type: "range",
                min: 0,
                max: 10,
                value: c().desired_one_way_connections as i64,
                oninput: move |event| {
                    c.set(MapConfig {
                        desired_one_way_connections: event
                            .value()
                            .parse()
                            .unwrap_or(c().desired_one_way_connections),
                        ..c()
                    })
                },
                id: "desired_one_way_connections"
            }
            label { "for": "desired_one_way_connections",
                "{c().desired_one_way_connections} - Desired One Way Connections"
            }
            br {}

            input {
                r#type: "checkbox",
                checked: c().portal_only_one_ways,
                id: "portal_only_one_ways",
                oninput: move |event| {
                    c.set(MapConfig {
                        portal_only_one_ways: event.value().parse().unwrap_or(false),
                        ..c()
                    })
                }
            }
            label { "for": "portal_only_one_ways", "One ways only on portal doors." }
            br {}

            input {
                r#type: "range",
                min: 0,
                max: 1,
                step: 0.01,
                value: c().decoration_odds,
                oninput: move |event| {
                    c.set(MapConfig {
                        decoration_odds: event.value().parse().unwrap_or(c().decoration_odds),
                        ..c()
                    })
                },
                id: "decoration_odds"
            }
            label { "for": "decoration_odds", "{c().decoration_odds * 100.0:1.0}% - Room Decoration Odds" }
            br {}

            input {
                r#type: "range",
                min: 0,
                max: 1,
                step: 0.01,
                value: c().chip_odds,
                oninput: move |event| {
                    c.set(MapConfig {
                        chip_odds: event.value().parse().unwrap_or(c().chip_odds),
                        ..c()
                    })
                },
                id: "chip_odds"
            }
            label { "for": "chip_odds", "{c().chip_odds * 100.0:1.0}% - Room Chip Odds" }
            br {}

            input {
                r#type: "range",
                min: 0,
                max: 1,
                step: 0.01,
                value: c().empty_room_odds,
                oninput: move |event| {
                    c.set(MapConfig {
                        empty_room_odds: event.value().parse().unwrap_or(c().empty_room_odds),
                        ..c()
                    })
                },
                id: "empty_room_odds"
            }
            label { "for": "empty_room_odds", "{c().empty_room_odds * 100.0:1.0}% - Empty Room Odds" }
            br {}
        }
    }
}

pub fn patch_file(
    name: &str,
    content: &[u8],
    cfg: Config,
    error_string: Signal<String>,
) -> Result<(), GuiError> {
    let patcher = setup(&cfg)?;
    let rom = patcher.patch_u8_vec(content)?;
    trigger_download(name, rom, error_string)?;
    Ok(())
}

//borrowed from dioxus discord user knickish
pub fn trigger_download(
    name: &str,
    content: Vec<u8>,
    mut error_string: Signal<String>,
) -> Result<(), GuiError> {
    let eval = eval(
        r#"
        let filename = await dioxus.recv();
        let content = await dioxus.recv();
        let data = new Uint8Array(content)
        
        var contentType = 'application/octet-stream';
        var a = document.createElement('a');
        var blob = new Blob([data], {'type':contentType});
        a.href = window.URL.createObjectURL(blob);
        a.download = filename;
        a.click();
        "#,
    );

    eval.send(name.into())?;
    eval.send(content.to_owned().into())?;

    use_future(move || {
        to_owned![eval];
        async move {
            if eval.join().await.is_err() {
                error_string.set("Error evaluating JS within trigger download".to_string());
            }
        }
    });
    Ok(())
}

fn setup(cfg: &Config) -> Result<Patcher, TGLError> {
    let mut patcher = Patcher::new();

    generate(&mut patcher, cfg)?;

    Ok(patcher)
}
