pub struct Config {
    pub(crate) corridor_config: CorridorConfig,
    pub(crate) log: bool,
}

pub struct CorridorConfig {
    pub(crate) shuffle_skies: bool,
    #[allow(dead_code)]
    pub(crate) shuffle_ground: bool,
    pub(crate) shuffle_corridors: bool,
    pub(crate) shuffle_bosses: bool,
    pub(crate) shuffle_final_boss: bool,
}
