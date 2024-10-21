use dioxus::prelude::*;
use tgl_rando_core::config::MapConfig;

//currently excluded are the two shop sizes
#[component]
pub fn map_config(c: Signal<MapConfig>) -> Element {
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
