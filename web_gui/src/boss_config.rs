use dioxus::prelude::*;
use tgl_rando_core::config::BossConfig;

//pub shuffle_bosses: bool,
//pub shuffle_final_boss: bool,
//pub rebalance_bosses: bool,
//pub randomize_boss_health: bool,
#[component]
pub fn boss_config(c: Signal<BossConfig>) -> Element {
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

            input {
                r#type: "checkbox",
                checked: c().allow_missingno,
                id: "allow_missingno",
                oninput: move |event| {
                    c.set(BossConfig {
                        allow_missingno: event.value().parse().unwrap_or(false),
                        ..c()
                    })
                }
            }
            label { "for": "allow_missingno",
                "Allow Missingno (shuffle in the skull miniboss even though it will have broken graphics)"
            }
            br {}
        }
    }
}
