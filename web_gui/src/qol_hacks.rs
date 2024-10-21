/*faster_starting_fire: true,
fix_hyper_laser: true,
enemy_erasers_unlocked_from_start: true,
remove_flash: true,*/
use dioxus::prelude::*;
use tgl_rando_core::config::QOLHacks;

#[component]
pub fn qol_hacks(c: Signal<QOLHacks>) -> Element {
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

            input {
                r#type: "checkbox",
                checked: c().always_go_fast,
                id: "always_go_fast",
                oninput: move |event| {
                    c.set(QOLHacks {
                        always_go_fast: event.value().parse().unwrap_or(false),
                        ..c()
                    })
                }
            }
            label { "for": "always_go_fast", "Always Go Fast (Have 5 shield movement at 1 shield.)" }
            br {}
        }
    }
}
