use dioxus::prelude::*;
use tgl_rando_core::config::{ColorOptions, ColorStrategy, HueOptions, SaturationOptions};

#[component]
pub fn hue_config(c: Signal<ColorOptions>, h: Signal<HueOptions>) -> Element {
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
