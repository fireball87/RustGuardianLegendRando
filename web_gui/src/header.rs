// https://discord.gg/jx6KE4jvJE

use dioxus::prelude::*;

#[component]
pub fn header() -> Element {
    rsx! {
        div {
            h1 { "Guardian Legend Randomizer" }
            a { href: "https://discord.gg/jx6KE4jvJE", "Discord" }
            " "
            a { href: "https://github.com/fireball87/RustGuardianLegendRando", "Source" }
        }
    }
}
