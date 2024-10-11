use crate::patcher::Patcher;
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

pub struct ItemGenerator;

impl ItemGenerator {
    pub fn prepare_items(
        patcher: &mut Patcher,
        multi_shops: usize,
        single_shops: usize,
        weapon_size: usize,
        blue: usize,
        red: usize,
        shield: usize,
        force_shields: bool,
        guns: usize,
        rapid_fires: usize,
        etanks: usize,
        enemy_erasers: usize,
        log: bool,
        rng: &mut ChaCha8Rng,
    ) -> (Vec<Vec<String>>, Vec<Vec<String>>, Vec<Vec<String>>) {
        let mut single_shop_library = vec![vec![]; 11];
        let mut multi_shop_library = vec![vec![]; 11];
        let mut item_library = vec![vec![]; 11];

        let mut item_pool = Self::create_item_pool(
            weapon_size,
            blue,
            red,
            shield,
            force_shields,
            guns,
            rapid_fires,
            etanks,
            enemy_erasers,
            rng,
        );

        let pool_size = item_pool.len();
        for i in 20..=pool_size - (single_shops + multi_shops + 1) {
            if !(30..52).contains(&i) {
                let area = rng.gen_range(0..=10);
                item_library[area].push(format!("{:02X}", i - 19));
                if log {
                    println!("item box {:02X} is in area {}", i - 19, area);
                }
            }
        }

        if force_shields {
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
                    possibilities.push(usize::from_str_radix(item, 16).unwrap() + 19);
                }
                for item in &item_library[x * 2] {
                    possibilities.push(usize::from_str_radix(item, 16).unwrap() + 19);
                }

                let index_to_swap = possibilities[rng.gen_range(0..possibilities.len())];
                item_pool.swap(pool_size - x, index_to_swap);
            }
        }

        let mut patch_string = String::new();
        if log {
            println!();
        }
        for i in 0..=19 {
            if log {
                println!("corridor {} has {}", i + 1, item_pool[i]);
            }
            patch_string.push_str(&item_pool[i]);
        }
        patcher.add_change(&patch_string, "1EF51");

        let mut item_string = String::new();

        for (i, item) in item_pool
            .iter()
            .enumerate()
            .take((pool_size - (single_shops + multi_shops + 1)) + 1)
            .skip(20)
        {
            item_string.push_str(item);

            if (30..52).contains(&i) {
                if log {
                    println!("miniboss {:02X} has {}", i - 19, item);
                }
            } else {
                if log {
                    println!("item box {:02X} has {}", i - 19, item);
                }
                if i - 19 > 57 {
                    panic!("Tried to place more item boxes than the game had");
                }
            }
        }

        patcher.add_change(&item_string, "16388");

        let mut patch_string = String::new();
        for (i, item) in item_pool
            .iter()
            .enumerate()
            .take((pool_size - (multi_shops + 1)) + 1)
            .skip(pool_size - (single_shops + multi_shops + 1) + 1)
        {
            let id = i - (pool_size - (single_shops + multi_shops + 1)) + 57;
            let price = Self::random_price_for_area(
                i - (pool_size - (single_shops + multi_shops + 1) + 1),
                rng,
            );

            let price_hex = format!("{:04X}", price);
            let flipped_price = format!("{}{}", &price_hex[2..4], &price_hex[0..2]);

            patch_string.push_str(&flipped_price);
            patch_string.push_str(item);
            single_shop_library[0].push(format!("{:02X}", id));
            if log {
                println!("small shop {:02X} has {}", id, item);
            }
        }
        patcher.add_change(&patch_string, "16077");

        let mut patch_string = String::new();
        for (i, item) in item_pool
            .iter()
            .enumerate()
            .take(pool_size)
            .skip(pool_size - (multi_shops + 1) + 1)
        {
            let id = i - (pool_size - (multi_shops + 1)) + 62;
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

            if log {
                println!("big shop {:02X} has {} in area {}", id, item, desired_area);
            }
        }

        patcher.add_change(&patch_string, "1605e");

        (item_library, single_shop_library, multi_shop_library)
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

    fn create_item_pool(
        weapon_size: usize,
        blue: usize,
        red: usize,
        shield: usize,
        force_shields: bool,
        guns: usize,
        rapid_fires: usize,
        etanks: usize,
        enemy_erasers: usize,
        rng: &mut ChaCha8Rng,
    ) -> Vec<String> {
        let mut pool = vec![];
        for _ in 0..weapon_size {
            for i in 0..=10 {
                pool.push(format!("{:02X}", i));
            }
        }
        for _ in 0..blue {
            pool.push(format!("{:02X}", Item::BlueLander as i32));
        }
        for _ in 0..red {
            pool.push(format!("{:02X}", Item::RedLander as i32));
        }

        let mut shield = shield;
        if force_shields {
            shield -= 5;
        }
        for _ in 0..shield {
            pool.push(format!("{:02X}", Item::Shield as i32));
        }

        for _ in 0..guns {
            pool.push(format!("{:02X}", Item::Gun as i32));
        }
        for _ in 0..rapid_fires {
            pool.push(format!("{:02X}", Item::RapidFire as i32));
        }

        for _ in 0..etanks {
            pool.push(format!("{:02X}", Item::EnemyTank as i32));
        }

        for _ in 0..enemy_erasers {
            pool.push(format!("{:02X}", Item::EnemyEraser as i32));
        }

        if pool.len() < 50 {
            panic!("Not enough items to fill shops.");
        }
        if pool.len() > 57 + 30 + 10 {
            panic!("Too many items to place.");
        }
        pool.shuffle(rng);

        if force_shields {
            for _ in 0..5 {
                pool.push(format!("{:02X}", Item::Shield as i32));
            }
        }
        pool
    }
}
