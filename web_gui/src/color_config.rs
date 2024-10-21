use crate::hue_config::hue_config;
use dioxus::prelude::*;
use tgl_rando_core::config::{ColorOptions, ColorStrategy, HueOptions};

#[component]
pub fn color_config(c: Signal<ColorOptions>, h: Signal<HueOptions>) -> Element {
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
