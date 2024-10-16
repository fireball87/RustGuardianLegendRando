use crate::config::Config;
use crate::patcher::Patcher;
use crate::tgl_error::TGLError;
use rand::prelude::SliceRandom;
use rand::Rng;
use rand_chacha::ChaCha8Rng;

#[derive(Debug, Clone, Copy)]
enum Item {
    // MultiBullet = 0,
    // BackWave = 1,
    // WaveBeam = 2,
    // BulletShield = 3,
    // Grenade = 4,
    // Fireball = 5,
    // AreaBlaster = 6,
    // Repeller = 7,
    // HyperLaser = 8,
    // SaberLaser = 9,
    // CutterSaber = 10,
    EnemyEraser = 11,
    EnemyTank = 12,
    BlueLander = 13,
    Gun = 14,
    Shield = 15,
    RapidFire = 16,
    RedLander = 17,
}

pub struct ItemLibrary {
    pub single_shop_library: Vec<Vec<String>>,
    pub multi_shop_library: Vec<Vec<String>>,
    pub item_library: Vec<Vec<String>>,
}
pub struct ItemGenerator;

impl ItemGenerator {
    pub fn prepare_items(
        patcher: &mut Patcher,
        cfg: &Config,
        rng: &mut ChaCha8Rng,
    ) -> Result<ItemLibrary, TGLError> {
        let mut single_shop_library = vec![vec![]; 11];
        let mut multi_shop_library = vec![vec![]; 11];
        let mut item_library = vec![vec![]; 11];

        let mut item_pool = Self::create_item_pool(cfg, rng)?;

        let pool_size = item_pool.len();
        for i in 20..=pool_size - (cfg.map_config.single_shops + cfg.map_config.multi_shops + 1) {
            if !(30..52).contains(&i) {
                let area = rng.gen_range(0..=10);
                item_library[area].push(format!("{:02X}", i - 19));
                if cfg.log {
                    println!("item box {:02X} is in area {}", i - 19, area);
                }
            }
        }

        if cfg.item_config.force_shields {
            for x in 1..=5 {
                let mut possibilities = vec![
                    x * 2 - 2,
                    x * 2 - 1,
                    10 + x * 2 - 2,
                    10 + x * 2 - 1,
                    31 + x * 4 - 3,
                    31 + x * 4 - 2,
                    31 + x * 4 - 1,
                    31 + x * 4,
                ];

                for item in &item_library[x * 2 - 1] {
                    possibilities.push(usize::from_str_radix(item, 16)? + 19);
                }
                for item in &item_library[x * 2] {
                    possibilities.push(usize::from_str_radix(item, 16)? + 19);
                }

                let index_to_swap = possibilities[rng.gen_range(0..possibilities.len())];
                item_pool.swap(pool_size - x, index_to_swap);
            }
        }

        let mut patch_string = String::new();
        if cfg.log {
            println!();
        }
        for (i, item) in item_pool.iter().enumerate().take(19 + 1) {
            if cfg.log {
                println!("corridor {} has {}", i + 1, item);
            }
            patch_string.push_str(item);
        }
        patcher.add_change(&patch_string, "1EF51");

        let mut item_string = String::new();

        for (i, item) in item_pool
            .iter()
            .enumerate()
            .take((pool_size - (cfg.map_config.single_shops + cfg.map_config.multi_shops + 1)) + 1)
            .skip(20)
        {
            item_string.push_str(item);

            if (30..52).contains(&i) {
                if cfg.log {
                    println!("miniboss {:02X} has {}", i - 19, item);
                }
            } else {
                if cfg.log {
                    println!("item box {:02X} has {}", i - 19, item);
                }
                if i - 19 > 57 {
                    return Err("Tried to place more item boxes than the game had".into());
                }
            }
        }

        patcher.add_change(&item_string, "16388");

        let mut patch_string = String::new();
        for (i, item) in item_pool
            .iter()
            .enumerate()
            .take((pool_size - (cfg.map_config.multi_shops + 1)) + 1)
            .skip(pool_size - (cfg.map_config.single_shops + cfg.map_config.multi_shops + 1) + 1)
        {
            let id = i
                - (pool_size - (cfg.map_config.single_shops + cfg.map_config.multi_shops + 1))
                + 57;
            let price = Self::random_price_for_area(
                i - (pool_size - (cfg.map_config.single_shops + cfg.map_config.multi_shops + 1)
                    + 1),
                rng,
            );

            let price_hex = format!("{:04X}", price);
            let flipped_price = format!("{}{}", &price_hex[2..4], &price_hex[0..2]);

            patch_string.push_str(&flipped_price);
            patch_string.push_str(item);
            single_shop_library[0].push(format!("{:02X}", id));
            if cfg.log {
                println!("small shop {:02X} has {}", id, item);
            }
        }
        patcher.add_change(&patch_string, "16077");

        let mut patch_string = String::new();
        for (i, item) in item_pool
            .iter()
            .enumerate()
            .take(pool_size)
            .skip(pool_size - (cfg.map_config.multi_shops + 1) + 1)
        {
            let id = i - (pool_size - (cfg.map_config.multi_shops + 1)) + 62;
            let desired_area = rng.gen_range(1..=10);
            let price = Self::random_price_for_area(desired_area, rng);

            let price_hex = format!("{:04X}", price);
            let flipped_price = format!("{}{}", &price_hex[2..4], &price_hex[0..2]);

            let rand_item0 = if rng.gen_range(0..=5) <= 2 { 12 } else { 11 };

            patch_string.push_str(&flipped_price);
            patch_string.push_str(item);
            patch_string.push_str(&format!("{:02X}", rand_item0));
            patch_string.push_str(&format!("{:02X}", rng.gen_range(0..=10)));
            multi_shop_library[desired_area].push(format!("{:02X}", id));

            if cfg.log {
                println!("big shop {:02X} has {} in area {}", id, item, desired_area);
            }
        }

        patcher.add_change(&patch_string, "1605e");

        Ok(ItemLibrary {
            item_library,
            single_shop_library,
            multi_shop_library,
        })
    }

    fn random_price_for_area(area: usize, rng: &mut ChaCha8Rng) -> usize {
        match area {
            0 => rng.gen_range(0..51),
            1 => rng.gen_range(50..101),
            2 => rng.gen_range(100..151),
            3 => rng.gen_range(150..301),
            4 => rng.gen_range(300..451),
            5 => rng.gen_range(450..601),
            6 => rng.gen_range(600..751),
            7 => rng.gen_range(750..1001),
            8 => rng.gen_range(1000..1601),
            9 => rng.gen_range(1600..2401),
            10 => rng.gen_range(2400..4001),
            _ => 0,
        }
    }

    fn create_item_pool(cfg: &Config, rng: &mut ChaCha8Rng) -> Result<Vec<String>, TGLError> {
        let item_cfg = &cfg.item_config;
        let mut pool = vec![];
        for _ in 0..item_cfg.weapon_size {
            for i in 0..=10 {
                pool.push(format!("{:02X}", i));
            }
        }
        for _ in 0..item_cfg.blue {
            pool.push(format!("{:02X}", Item::BlueLander as i32));
        }
        for _ in 0..item_cfg.red {
            pool.push(format!("{:02X}", Item::RedLander as i32));
        }

        let mut shield = item_cfg.shield;
        if item_cfg.force_shields {
            shield -= 5;
        }
        for _ in 0..shield {
            pool.push(format!("{:02X}", Item::Shield as i32));
        }

        for _ in 0..item_cfg.guns {
            pool.push(format!("{:02X}", Item::Gun as i32));
        }
        for _ in 0..item_cfg.rapid_fires {
            pool.push(format!("{:02X}", Item::RapidFire as i32));
        }

        for _ in 0..item_cfg.etanks {
            pool.push(format!("{:02X}", Item::EnemyTank as i32));
        }

        for _ in 0..item_cfg.enemy_erasers {
            pool.push(format!("{:02X}", Item::EnemyEraser as i32));
        }

        if pool.len() < 50 {
            return Err("Not enough items to fill shops.".into());
        }
        if pool.len() > 57 + 30 + 10 {
            return Err("Too many items to place.".into());
        }
        pool.shuffle(rng);

        if item_cfg.force_shields {
            for _ in 0..5 {
                pool.push(format!("{:02X}", Item::Shield as i32));
            }
        }
        Ok(pool)
    }
}
