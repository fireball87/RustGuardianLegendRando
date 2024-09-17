use dioxus::prelude::*;
use tgl_rando_core::config::*;
use tgl_rando_core::patcher::Patcher;
use tgl_rando_core::{generate, patcher, seed};

/*
#[derive(Props, PartialEq, Clone)]
pub struct HueOptionsWrapper {
    c: config::HueOptions
}

impl Default for HueOptionsWrapper {
    fn default() -> Self {
        HueOptionsWrapper
        {
            c: config::HueOptions::default()
        }
    }
}


#[derive(Props, PartialEq, Clone)]
pub struct QOLHacksWrapper {
    c: config::QOLHacks
}

impl Default for QOLHacksWrapper {
    fn default() -> Self {
        QOLHacksWrapper {
            c: config::QOLHacks::default()
        }
    }
}*/

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
    let corridor_cfg = use_signal(|| CorridorConfig::default());
    let boss_cfg = use_signal(|| BossConfig::default());
    let qol_cfg = use_signal(|| QOLHacks::default());

    rsx! {
        div {
            input {
                // tell the input to pick a file
                r#type: "file",
                // list the accepted extensions
                accept: ".rom,.nes,.bin",
                // pick multiple files
                multiple: false,
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
            corridor_config {c: corridor_cfg}
            boss_config {c: boss_cfg}
            qol_hacks {c: qol_cfg}
            button {
                disabled: uploaded().len() == 0,
                onclick: move |_| {
                    patch_file(
                        "tgl_rando.nes",
                        &(uploaded.read()),
                        Config {
                            corridor_config: corridor_cfg(),
                            qol_hacks: qol_cfg(),
                            color_strategy: ColorStrategy::Vanilla(HueOptions::default()),
                            boss_config: boss_cfg(),
                            log: false,
                            seed: seed::make_seed()
                        }
                    )
                },
                "Generate"
            }
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
            h3 {"Corridor Config"}
            input {
                r#type: "checkbox",
                checked:c().shuffle_corridors,
                id: "shuffleCorridors",
                oninput: move |event| c.set(CorridorConfig {shuffle_corridors: event.value().parse().unwrap(), ..c()
                })
            }
            label {for: "shuffleCorridors", "Shuffle Corridors"}
            br {}

            input {
                r#type: "checkbox",
                checked: c().shuffle_ground,
                id: "shuffle_ground",
                oninput: move |event| c.set(CorridorConfig {shuffle_ground: event.value().parse().unwrap(), ..c()})
            }
            label {for: "shuffleGround", "Shuffle Ground !!!Currently Broken!!!"}
            br {}

            input {
                r#type: "checkbox",
                checked: c().shuffle_skies,
                id: "shuffle_skies",
                oninput: move |event| c.set(CorridorConfig {shuffle_skies: event.value().parse().unwrap(), ..c()})
            }
            label {for: "shuffleSkies", "Shuffle Air Enemy Spawns"}
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
            h3 {"Boss Config"}

            input {
                r#type: "checkbox",
                checked:c().shuffle_bosses,
                id: "shuffle_bosses",
                oninput: move |event| c.set(BossConfig {shuffle_bosses: event.value().parse().unwrap(), ..c()
                })
            }
            label {for: "shuffle_bosses", "Shuffle Bosses"}
            br {}


            input {
                r#type: "checkbox",
                checked: c().shuffle_final_boss,
                id: "shuffle_final_boss",
                oninput: move |event| c.set(BossConfig {shuffle_final_boss: event.value().parse().unwrap(), ..c()})
            }
            label {for: "shuffle_final_boss", "Shuffle Final Boss (if shuffling bosses)"}
            br {}


            input {
                r#type: "checkbox",
                checked: c().rebalance_bosses,
                id: "rebalance_bosses",
                oninput: move |event| c.set(BossConfig {rebalance_bosses: event.value().parse().unwrap(), ..c()})
            }
            label {for: "rebalance_bosses", "Rebalance Bosses and apply Scaling Hack (will act as if true if either Shuffle Corridor or Shuffle Bosses is true.)"}
            br {}

            input {
                r#type: "checkbox",
                checked: c().randomize_boss_health,
                id: "randomize_boss_health",
                oninput: move |event| c.set(BossConfig {randomize_boss_health: event.value().parse().unwrap(), ..c()})
            }
            label {for: "randomize_boss_health", "Randomize Boss Health Values (if we are rebalancing them)"}
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
            h3 {"Quality Of Life Hacks"}

            input {
                r#type: "checkbox",
                checked:c().faster_starting_fire,
                id: "faster_starting_fire",
                oninput: move |event| c.set(QOLHacks {faster_starting_fire: event.value().parse().unwrap(), ..c()
                })
            }
            label {for: "faster_starting_fire", "Increase Starting Fire Rate"}
            br {}

            input {
                r#type: "checkbox",
                checked:c().fix_hyper_laser,
                id: "fix_hyper_laser",
                oninput: move |event| c.set(QOLHacks {fix_hyper_laser: event.value().parse().unwrap(), ..c()
                })
            }
            label {for: "fix_hyper_laser", "Buff Hyperlaser Damage"}
            br {}

            input {
                r#type: "checkbox",
                checked:c().enemy_erasers_unlocked_from_start,
                id: "enemy_erasers_unlocked_from_start",
                oninput: move |event| c.set(QOLHacks {enemy_erasers_unlocked_from_start: event.value().parse().unwrap(), ..c()
                })
            }
            label {for: "enemy_erasers_unlocked_from_start", "Unlock Enemy Eraser Drops At Game Start"}
            br {}

            input {
                r#type: "checkbox",
                checked:c().remove_flash,
                id: "remove_flash",
                oninput: move |event| c.set(QOLHacks {remove_flash: event.value().parse().unwrap(), ..c()
                })
            }
            label {for: "remove_flash", "Remove Flashing (enemy erasers and boss kills)"}
            br {}


        }
    }
}

pub fn patch_file(name: &str, content: &Vec<u8>, cfg: Config) {
    let patcher = setup(&cfg);
    let rom = patcher.patch_u8_vec(content);
    trigger_download(name, rom);
}

//borrowed from dioxus discord user knickish
pub fn trigger_download(name: &str, content: Vec<u8>) {
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

    eval.send(name.into()).unwrap();
    eval.send(content.to_owned().into()).unwrap();

    use_future(move || {
        to_owned![eval];
        async move {
            eval.join().await.unwrap();
        }
    });
}

fn setup(cfg: &Config) -> Patcher {
    let mut patcher = Patcher::new();

    generate(&mut patcher, cfg);

    patcher
}