use crate::seed;

#[allow(dead_code)]
#[derive(PartialEq, Eq, Clone)]
pub enum ColorStrategy {
    Vanilla(HueOptions),
    All(HueOptions),
    Random,
    ColorTheory(HueOptions),
}

#[derive(PartialEq, Eq, Clone)]
pub struct ColorOptions {
    pub color_strategy: ColorStrategy,
    pub include_foreground: bool,
}

impl Default for ColorOptions {
    fn default() -> Self {
        ColorOptions {
            color_strategy: ColorStrategy::All(HueOptions::default()),
            include_foreground: true,
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum SaturationOptions {
    None,
    Safe,
    All,
}
#[derive(PartialEq, Eq, Clone)]
pub struct HueOptions {
    pub rotate_hue: bool,
    pub flip_saturation: SaturationOptions,
}

impl Default for HueOptions {
    fn default() -> Self {
        HueOptions {
            rotate_hue: true,
            flip_saturation: SaturationOptions::None,
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct CorridorConfig {
    pub shuffle_skies: bool,
    pub shuffle_ground: bool,
    pub shuffle_corridors: bool,
}
impl Default for CorridorConfig {
    fn default() -> Self {
        CorridorConfig {
            shuffle_skies: true,
            shuffle_ground: true,
            shuffle_corridors: true,
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct QOLHacks {
    pub faster_starting_fire: bool,
    pub fix_hyper_laser: bool,
    pub enemy_erasers_unlocked_from_start: bool,
    pub remove_flash: bool,
}

impl Default for QOLHacks {
    fn default() -> Self {
        QOLHacks {
            faster_starting_fire: true,
            fix_hyper_laser: true,
            enemy_erasers_unlocked_from_start: true,
            remove_flash: true,
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct BossConfig {
    pub rebalance_bosses: bool,
    pub randomize_boss_health: bool,
    pub shuffle_bosses: bool,
    pub shuffle_final_boss: bool,
}

impl Default for BossConfig {
    fn default() -> Self {
        BossConfig {
            rebalance_bosses: true,
            randomize_boss_health: true,
            shuffle_bosses: true,
            shuffle_final_boss: true,
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Config {
    pub corridor_config: CorridorConfig,
    pub qol_hacks: QOLHacks,
    // pub bad_ideas: BadIdeas,
    pub color_options: ColorOptions,
    pub boss_config: BossConfig,
    pub log: bool,
    pub seed: String,
}

impl Default for Config {
    fn default() -> Self {
        let rng_seed = seed::make_seed();

        Config {
            corridor_config: CorridorConfig::default(),
            qol_hacks: QOLHacks::default(),
            color_options: ColorOptions::default(),
            boss_config: BossConfig::default(),
            log: true,
            seed: rng_seed,
        }
    }
}
