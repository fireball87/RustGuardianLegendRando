pub struct Config {
    pub(crate) corridor_config: CorridorConfig,
    pub(crate) qol_hacks: QOLHacks,
    pub(crate) rebalance_bosses: bool,
    pub(crate) randomize_boss_health: bool,
    pub(crate) log: bool,
    pub seed: String,
}

pub struct CorridorConfig {
    pub(crate) shuffle_skies: bool,
    #[allow(dead_code)]
    pub(crate) shuffle_ground: bool,
    pub(crate) shuffle_corridors: bool,
    pub(crate) shuffle_bosses: bool,
    pub(crate) shuffle_final_boss: bool,
}

pub struct QOLHacks {
    pub(crate) faster_starting_fire: bool,
    pub(crate) fix_hyper_laser: bool,
    pub(crate) enemy_erasers_unlocked_from_start: bool,
    pub(crate) remove_flash: bool,
}
