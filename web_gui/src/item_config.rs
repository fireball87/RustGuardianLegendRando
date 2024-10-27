use dioxus::prelude::*;
use tgl_rando_core::config::ItemConfig;

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
pub fn item_config(c: Signal<ItemConfig>) -> Element {
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
                min: if c().force_shields { 5 } else { 0 },
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
                disabled: c().shield < 5,
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
