use dioxus::prelude::*;
use tgl_rando_core::config::{MusicConfig, MusicOptions};

#[component]
pub fn music_config(c: Signal<MusicConfig>) -> Element {
    rsx! {
        h3 { "Music Config" }
        select {
            id: "corridor",
            oninput: move |event| {
                match &*event.value() {
                    "Untouched" => {
                        c.set(MusicConfig {
                            corridor_music: MusicOptions::Untouched,
                            ..c()
                        })
                    }
                    "CorridorTracks" => {
                        c.set(MusicConfig {
                            corridor_music: MusicOptions::Corridor,
                            ..c()
                        })
                    }
                    "OverworldTracks" => {
                        c.set(MusicConfig {
                            corridor_music: MusicOptions::Overworld,
                            ..c()
                        })
                    }
                    "CorridorAndOverworld" => {
                        c.set(MusicConfig {
                            corridor_music: MusicOptions::CorridorAndOverworld,
                            ..c()
                        })
                    }
                    _ => {}
                }
            },
            option { value: "Untouched", label: "Untouched", selected: c().corridor_music == MusicOptions::Untouched }
            option {
                value: "CorridorTracks",
                label: "Random Corridor Tracks",
                selected: c().corridor_music == MusicOptions::Corridor
            }
            option {
                value: "OverworldTracks",
                label: "Random Overworld Tracks",
                selected: c().corridor_music == MusicOptions::Overworld
            }
            option {
                value: "CorridorAndOverworld",
                label: "Random Corridor or Overworld Tracks",
                selected: c().corridor_music == MusicOptions::CorridorAndOverworld
            }
        }
        label { "for": "corridor", "Corridor Music" }
        br {}

        select {
            id: "overworld",
            oninput: move |event| {
                match &*event.value() {
                    "Untouched" => {
                        c.set(MusicConfig {
                            area_music: MusicOptions::Untouched,
                            ..c()
                        })
                    }
                    "CorridorTracks" => {
                        c.set(MusicConfig {
                            area_music: MusicOptions::Corridor,
                            ..c()
                        })
                    }
                    "OverworldTracks" => {
                        c.set(MusicConfig {
                            area_music: MusicOptions::Overworld,
                            ..c()
                        })
                    }
                    "CorridorAndOverworld" => {
                        c.set(MusicConfig {
                            area_music: MusicOptions::CorridorAndOverworld,
                            ..c()
                        })
                    }
                    _ => {}
                }
            },
            option { value: "Untouched", label: "Untouched", selected: c().area_music == MusicOptions::Untouched }
            option {
                value: "CorridorTracks",
                label: "Random Corridor Tracks",
                selected: c().area_music == MusicOptions::Corridor
            }
            option {
                value: "OverworldTracks",
                label: "Random Overworld Tracks",
                selected: c().area_music == MusicOptions::Overworld
            }
            option {
                value: "CorridorAndOverworld",
                label: "Random Corridor or Overworld Tracks",
                selected: c().area_music == MusicOptions::CorridorAndOverworld
            }
        }
        label { "for": "overworld", "Overworld Music" }
        br {}
    }
}
