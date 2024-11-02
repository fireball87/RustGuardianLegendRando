mod boss_config;
mod color_config;
mod corridor_config;
mod gui_error;
mod header;
mod hue_config;
mod item_config;
mod map_config;
mod music_config;
mod qol_hacks;

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
    let music_cfg = use_signal(MusicConfig::default);
    let hue_cfg = use_signal(HueOptions::default);

    let mut advanced_visible = use_signal(|| false);
    let map_cfg = use_signal(MapConfig::default);
    let item_cfg = use_signal(ItemConfig::default);

    let mut err_string = use_signal(String::new);

    rsx! {
        div {
            header::header {}

            corridor_config::corridor_config { c: corridor_cfg }
            boss_config::boss_config { c: boss_cfg }
            qol_hacks::qol_hacks { c: qol_cfg }
            color_config::color_config { c: color_cfg, h: hue_cfg }
            music_config::music_config { c: music_cfg }

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
                map_config::map_config { c: map_cfg }
                item_config::item_config { c: item_cfg }
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
                            music_config: music_cfg(),
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
