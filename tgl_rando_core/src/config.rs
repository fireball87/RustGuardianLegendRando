#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum ColorStrategy {
    Vanilla(HueOptions),
    All(HueOptions),
    Random,
    ColorTheory(HueOptions),
}

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum SaturationOptions {
    None,
    Safe,
    All,
}
#[derive(PartialEq, Eq)]
pub struct HueOptions {
    pub rotate_hue: bool,
    pub flip_saturation: SaturationOptions,
}

pub struct Config {
    pub corridor_config: CorridorConfig,
    pub qol_hacks: QOLHacks,
    // pub bad_ideas: BadIdeas,
    pub color_strategy: ColorStrategy,
    pub rebalance_bosses: bool,
    pub randomize_boss_health: bool,
    pub log: bool,
    pub seed: String,
}

/*pub struct BadIdeas {

}*/

pub struct CorridorConfig {
    pub shuffle_skies: bool,
    pub shuffle_ground: bool,
    pub shuffle_corridors: bool,
    pub shuffle_bosses: bool,
    pub shuffle_final_boss: bool,
}

pub struct QOLHacks {
    pub faster_starting_fire: bool,
    pub fix_hyper_laser: bool,
    pub enemy_erasers_unlocked_from_start: bool,
    pub remove_flash: bool,
}
