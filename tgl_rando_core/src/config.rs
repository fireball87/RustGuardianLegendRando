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
    pub always_go_fast: bool,
}

impl Default for QOLHacks {
    fn default() -> Self {
        QOLHacks {
            faster_starting_fire: true,
            fix_hyper_laser: true,
            enemy_erasers_unlocked_from_start: true,
            remove_flash: true,
            always_go_fast: false,
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct BossConfig {
    pub rebalance_bosses: bool,
    pub randomize_boss_health: bool,
    pub shuffle_bosses: bool,
    pub shuffle_final_boss: bool,
    pub allow_missingno: bool,
}

impl Default for BossConfig {
    fn default() -> Self {
        BossConfig {
            rebalance_bosses: true,
            randomize_boss_health: true,
            shuffle_bosses: true,
            shuffle_final_boss: true,
            allow_missingno: false,
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct MapConfig {
    pub min_area_size: usize,
    pub max_area_size: usize,
    pub desired_connections: i32,
    pub desired_one_way_connections: i32,
    pub portal_only_one_ways: bool,
    pub decoration_odds: f64, // from 0 to 1 as a percent
    pub chip_odds: f64,       // from 0 to 1 as a percent
    pub empty_room_odds: f64, // from 0 to 1 as a percent
    pub multi_shops: usize,
    pub single_shops: usize,
}

impl Default for MapConfig {
    fn default() -> Self {
        MapConfig {
            min_area_size: 18,
            max_area_size: 25,
            desired_connections: 3,
            desired_one_way_connections: 0,
            portal_only_one_ways: false,
            decoration_odds: 0.17,
            chip_odds: 0.3,
            empty_room_odds: 0.1,
            multi_shops: 5,
            single_shops: 5,
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct ItemConfig {
    pub weapon_size: usize,
    pub blue: usize,
    pub red: usize,
    pub shield: usize,
    pub force_shields: bool,
    pub guns: usize,
    pub rapid_fires: usize,
    pub etanks: usize,
    pub enemy_erasers: usize,
}

impl Default for ItemConfig {
    fn default() -> Self {
        ItemConfig {
            weapon_size: 4,
            blue: 9,
            red: 10,
            shield: 6,
            force_shields: true,
            guns: 5,
            rapid_fires: 5,
            etanks: 3,
            enemy_erasers: 5,
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Config {
    pub corridor_config: CorridorConfig,
    pub map_config: MapConfig,
    pub item_config: ItemConfig,
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
            map_config: MapConfig::default(),
            item_config: ItemConfig::default(),
            qol_hacks: QOLHacks::default(),
            color_options: ColorOptions::default(),
            boss_config: BossConfig::default(),
            log: true,
            seed: rng_seed,
        }
    }
}
