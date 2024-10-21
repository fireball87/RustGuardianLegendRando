use dioxus::prelude::*;
use tgl_rando_core::config::CorridorConfig;

//pub shuffle_skies: bool,
//pub shuffle_ground: bool,
//pub shuffle_corridors: bool,
#[component]
pub fn corridor_config(c: Signal<CorridorConfig>) -> Element {
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
