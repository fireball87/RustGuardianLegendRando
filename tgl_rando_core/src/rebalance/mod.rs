mod damage;
mod entity;
mod health;

use crate::config::Config;
use crate::patcher::Patcher;
use rand::Rng;
use rand_chacha::ChaCha8Rng;

pub fn handle_rebalance(patcher: &mut Patcher, config: &Config, rng: &mut ChaCha8Rng) {
    if config.rebalance_bosses
        || config.corridor_config.shuffle_corridors
        || config.corridor_config.shuffle_bosses
    {
        let rand_health = config.randomize_boss_health;
        rebalance_all(patcher, rand_health, rng);
    }
}
fn rebalance_all(patcher: &mut Patcher, randomize_health: bool, rng: &mut ChaCha8Rng) {
    patcher.add_change("606060", "1c172");
    patcher.add_change("20a9ff", "1cfd0");
    patcher.add_change("9d20062088fe60", "1ffb9");

    shift_damage(patcher);
    shift_health(patcher, randomize_health, rng);
    shift_projectiles(patcher);
}

fn shift_projectiles(patcher: &mut Patcher) {
    //patch the health and damage tables to a normalized value
    patcher.add_change("1010101010101010", "1A227");
    patcher.add_change("0505050505050505", "1A21F");

    //then we need to patch the code to be balanced
    //patch the code we'll be jumping to
    //jsr $c0a2
    //20a2c0
    //jmp $FE88 , jmp because it saves me writing a return here as i can just piggy back fe88s ret
    //4c88fe
    patcher.add_change("20a2c04c88fe", "1ffc0");

    //patch the jump to this code, which is at
    patcher.add_change("20b0ff", "1a11a");
}

fn to_patch(input: u32) -> String {
    format!("{:02X}", input)
}

fn to_offset(input: u32) -> String {
    format!("{:06X}", input)
}
fn shift_damage(patcher: &mut Patcher) {
    let damage_offset: u32 = 119098;
    //bosses
    patcher.add_change(
        &to_patch(damage::EYEGORE_BLUE),
        &to_offset(damage_offset + entity::EYEGORE_BLUE),
    );
    patcher.add_change(
        &to_patch(damage::EYEGORE_RED),
        &to_offset(damage_offset + entity::EYEGORE_RED),
    );
    patcher.add_change(
        &to_patch(damage::ZIBZUB),
        &to_offset(damage_offset + entity::ZIBZUB),
    );
    patcher.add_change(
        &to_patch(damage::CLAWBOT_GREEN),
        &to_offset(damage_offset + entity::CLAWBOT_GREEN),
    );
    patcher.add_change(
        &to_patch(damage::CLAWBOT_BLUE),
        &to_offset(damage_offset + entity::CLAWBOT_BLUE),
    );
    patcher.add_change(
        &to_patch(damage::CLAWBOT_RED),
        &to_offset(damage_offset + entity::CLAWBOT_RED),
    );
    patcher.add_change(
        &to_patch(damage::BOMBARDER_RED),
        &to_offset(damage_offset + entity::BOMBARDER_RED),
    );
    patcher.add_change(
        &to_patch(damage::BOMBARDER_BLUE),
        &to_offset(damage_offset + entity::BOMBARDER_BLUE),
    );
    patcher.add_change(
        &to_patch(damage::OPTOMON_GREEN),
        &to_offset(damage_offset + entity::OPTOMON_GREEN),
    );
    patcher.add_change(
        &to_patch(damage::OPTOMON_BLUE),
        &to_offset(damage_offset + entity::OPTOMON_BLUE),
    );
    patcher.add_change(
        &to_patch(damage::OPTOMON_RED),
        &to_offset(damage_offset + entity::OPTOMON_RED),
    );
    patcher.add_change(
        &to_patch(damage::FLEEPA_BLUE),
        &to_offset(damage_offset + entity::FLEEPA_BLUE),
    );
    patcher.add_change(
        &to_patch(damage::FLEEPA_RED),
        &to_offset(damage_offset + entity::FLEEPA_RED),
    );
    patcher.add_change(
        &to_patch(damage::CRAWDADDY),
        &to_offset(damage_offset + entity::CRAWDADDY),
    );
    patcher.add_change(
        &to_patch(damage::TERRAMUTE),
        &to_offset(damage_offset + entity::TERRAMUTE),
    );
    patcher.add_change(
        &to_patch(damage::GLIDER),
        &to_offset(damage_offset + entity::GLIDER),
    );
    patcher.add_change(
        &to_patch(damage::GRIMGRIN_BLUE),
        &to_offset(damage_offset + entity::GRIMGRIN_BLUE),
    );
    patcher.add_change(
        &to_patch(damage::GRIMGRIN_RED),
        &to_offset(damage_offset + entity::GRIMGRIN_RED),
    );
    patcher.add_change(
        &to_patch(damage::IT),
        &to_offset(damage_offset + entity::IT),
    );

    //minibosses
    patcher.add_change(
        &to_patch(damage::SPIDER_GREEN),
        &to_offset(damage_offset + entity::SPIDER_GREEN),
    );
    patcher.add_change(
        &to_patch(damage::SPIDER_BLUE),
        &to_offset(damage_offset + entity::SPIDER_BLUE),
    );
    patcher.add_change(
        &to_patch(damage::SPIDER_RED),
        &to_offset(damage_offset + entity::SPIDER_RED),
    );
    patcher.add_change(
        &to_patch(damage::CRAB_GREEN),
        &to_offset(damage_offset + entity::CRAB_GREEN),
    );
    patcher.add_change(
        &to_patch(damage::CRAB_BLUE),
        &to_offset(damage_offset + entity::CRAB_BLUE),
    );
    patcher.add_change(
        &to_patch(damage::CRAB_RED),
        &to_offset(damage_offset + entity::CRAB_RED),
    );
    patcher.add_change(
        &to_patch(damage::CARPET),
        &to_offset(damage_offset + entity::CARPET),
    );
    patcher.add_change(
        &to_patch(damage::BOUNCER_GREEN),
        &to_offset(damage_offset + entity::BOUNCER_GREEN),
    );
    patcher.add_change(
        &to_patch(damage::BOUNCER_BLUE),
        &to_offset(damage_offset + entity::BOUNCER_BLUE),
    );
    patcher.add_change(
        &to_patch(damage::BOUNCER_RED),
        &to_offset(damage_offset + entity::BOUNCER_RED),
    );
    patcher.add_change(
        &to_patch(damage::CRYSTAL_STAR),
        &to_offset(damage_offset + entity::CRYSTAL_STAR),
    );
    patcher.add_change(
        &to_patch(damage::SKULL),
        &to_offset(damage_offset + entity::SKULL),
    );

    //normal enemies i think
    patcher.add_change(
        &to_patch(damage::RED_SKULL_ENEMY),
        &to_offset(damage_offset + entity::RED_SKULL_ENEMY),
    );
    patcher.add_change(
        &to_patch(damage::BLUE_SKULL_ENEMY),
        &to_offset(damage_offset + entity::BLUE_SKULL_ENEMY),
    );
}

fn shift_health(patcher: &mut Patcher, randomize_health: bool, rng: &mut ChaCha8Rng) {
    let health_offset: u32 = 118971;

    if randomize_health {
        patcher.add_change(
            &to_patch(rng.gen_range(12..=24)),
            &to_offset(health_offset + entity::EYEGORE_BLUE),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(12..=24)),
            &to_offset(health_offset + entity::EYEGORE_RED),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(12..=24)),
            &to_offset(health_offset + entity::ZIBZUB),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(12..=24)),
            &to_offset(health_offset + entity::CLAWBOT_GREEN),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(12..=24)),
            &to_offset(health_offset + entity::CLAWBOT_BLUE),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(12..=24)),
            &to_offset(health_offset + entity::CLAWBOT_RED),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(12..=24)),
            &to_offset(health_offset + entity::OPTOMON_GREEN),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(12..=24)),
            &to_offset(health_offset + entity::OPTOMON_BLUE),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(12..=24)),
            &to_offset(health_offset + entity::OPTOMON_RED),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(12..=24)),
            &to_offset(health_offset + entity::FLEEPA_BLUE),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(12..=24)),
            &to_offset(health_offset + entity::FLEEPA_RED),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(12..=24)),
            &to_offset(health_offset + entity::CRAWDADDY),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(12..=24)),
            &to_offset(health_offset + entity::TERRAMUTE),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(4..=8)),
            &to_offset(health_offset + entity::GLIDER),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(22..=35)),
            &to_offset(health_offset + entity::GRIMGRIN_BLUE),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(24..=37)),
            &to_offset(health_offset + entity::GRIMGRIN_RED),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(8..=12)),
            &to_offset(health_offset + entity::BOMBARDER_RED),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(8..=12)),
            &to_offset(health_offset + entity::BOMBARDER_BLUE),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(12..=24)),
            &to_offset(health_offset + entity::EYEGORE_BLUE),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(22..=35)),
            &to_offset(health_offset + entity::IT),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(4..=12)),
            &to_offset(health_offset + entity::SPIDER_GREEN),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(4..=12)),
            &to_offset(health_offset + entity::SPIDER_BLUE),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(4..=12)),
            &to_offset(health_offset + entity::SPIDER_RED),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(4..=12)),
            &to_offset(health_offset + entity::CRAB_GREEN),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(4..=12)),
            &to_offset(health_offset + entity::CRAB_BLUE),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(4..=12)),
            &to_offset(health_offset + entity::CRAB_RED),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(4..=12)),
            &to_offset(health_offset + entity::CARPET),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(4..=12)),
            &to_offset(health_offset + entity::BOUNCER_GREEN),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(4..=12)),
            &to_offset(health_offset + entity::BOUNCER_RED),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(4..=12)),
            &to_offset(health_offset + entity::BOUNCER_BLUE),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(4..=12)),
            &to_offset(health_offset + entity::CRYSTAL_STAR),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(4..=12)),
            &to_offset(health_offset + entity::SKULL),
        );

        patcher.add_change(
            &to_patch(rng.gen_range(4..=12)),
            &to_offset(health_offset + entity::GROUND_EYE_BIG),
        );
        patcher.add_change(
            &to_patch(rng.gen_range(4..=12)),
            &to_offset(health_offset + entity::GROUND_EYE_SMALL),
        );
    } else {
        patcher.add_change(
            &to_patch(health::EYEGORE_BLUE),
            &to_offset(health_offset + entity::EYEGORE_BLUE),
        );
        patcher.add_change(
            &to_patch(health::EYEGORE_RED),
            &to_offset(health_offset + entity::EYEGORE_RED),
        );
        patcher.add_change(
            &to_patch(health::ZIBZUB),
            &to_offset(health_offset + entity::ZIBZUB),
        );
        patcher.add_change(
            &to_patch(health::CLAWBOT_GREEN),
            &to_offset(health_offset + entity::CLAWBOT_GREEN),
        );
        patcher.add_change(
            &to_patch(health::CLAWBOT_BLUE),
            &to_offset(health_offset + entity::CLAWBOT_BLUE),
        );
        patcher.add_change(
            &to_patch(health::CLAWBOT_RED),
            &to_offset(health_offset + entity::CLAWBOT_RED),
        );
        patcher.add_change(
            &to_patch(health::BOMBARDER_RED),
            &to_offset(health_offset + entity::BOMBARDER_RED),
        );
        patcher.add_change(
            &to_patch(health::BOMBARDER_BLUE),
            &to_offset(health_offset + entity::BOMBARDER_BLUE),
        );
        patcher.add_change(
            &to_patch(health::OPTOMON_GREEN),
            &to_offset(health_offset + entity::OPTOMON_GREEN),
        );
        patcher.add_change(
            &to_patch(health::OPTOMON_BLUE),
            &to_offset(health_offset + entity::OPTOMON_BLUE),
        );
        patcher.add_change(
            &to_patch(health::OPTOMON_RED),
            &to_offset(health_offset + entity::OPTOMON_RED),
        );
        patcher.add_change(
            &to_patch(health::FLEEPA_BLUE),
            &to_offset(health_offset + entity::FLEEPA_BLUE),
        );
        patcher.add_change(
            &to_patch(health::FLEEPA_RED),
            &to_offset(health_offset + entity::FLEEPA_RED),
        );
        patcher.add_change(
            &to_patch(health::CRAWDADDY),
            &to_offset(health_offset + entity::CRAWDADDY),
        );
        patcher.add_change(
            &to_patch(health::TERRAMUTE),
            &to_offset(health_offset + entity::TERRAMUTE),
        );
        patcher.add_change(
            &to_patch(health::GLIDER),
            &to_offset(health_offset + entity::GLIDER),
        );
        patcher.add_change(
            &to_patch(health::GRIMGRIN_RED),
            &to_offset(health_offset + entity::GRIMGRIN_RED),
        );
        patcher.add_change(
            &to_patch(health::GRIMGRIN_BLUE),
            &to_offset(health_offset + entity::GRIMGRIN_BLUE),
        );
        patcher.add_change(
            &to_patch(health::IT),
            &to_offset(health_offset + entity::IT),
        );
        patcher.add_change(
            &to_patch(health::SPIDER_GREEN),
            &to_offset(health_offset + entity::SPIDER_GREEN),
        );
        patcher.add_change(
            &to_patch(health::SPIDER_BLUE),
            &to_offset(health_offset + entity::SPIDER_BLUE),
        );
        patcher.add_change(
            &to_patch(health::SPIDER_RED),
            &to_offset(health_offset + entity::SPIDER_RED),
        );
        patcher.add_change(
            &to_patch(health::CRAB_GREEN),
            &to_offset(health_offset + entity::CRAB_GREEN),
        );
        patcher.add_change(
            &to_patch(health::CRAB_BLUE),
            &to_offset(health_offset + entity::CRAB_BLUE),
        );
        patcher.add_change(
            &to_patch(health::CRAB_RED),
            &to_offset(health_offset + entity::CRAB_RED),
        );
        patcher.add_change(
            &to_patch(health::CARPET),
            &to_offset(health_offset + entity::CARPET),
        );
        patcher.add_change(
            &to_patch(health::BOUNCER_GREEN),
            &to_offset(health_offset + entity::BOUNCER_GREEN),
        );
        patcher.add_change(
            &to_patch(health::BOUNCER_BLUE),
            &to_offset(health_offset + entity::BOUNCER_BLUE),
        );
        patcher.add_change(
            &to_patch(health::BOUNCER_RED),
            &to_offset(health_offset + entity::BOUNCER_RED),
        );
        patcher.add_change(
            &to_patch(health::CRYSTAL_STAR),
            &to_offset(health_offset + entity::CRYSTAL_STAR),
        );
        patcher.add_change(
            &to_patch(health::SKULL),
            &to_offset(health_offset + entity::SKULL),
        );

        patcher.add_change(
            &to_patch(health::GROUND_EYE_SMALL),
            &to_offset(health_offset + entity::GROUND_EYE_SMALL),
        );
        patcher.add_change(
            &to_patch(health::GROUND_EYE_BIG),
            &to_offset(health_offset + entity::GROUND_EYE_BIG),
        );
    }
    patcher.add_change(
        &to_patch(health::DEF_LARGE),
        &to_offset(health_offset + entity::DEF_LARGE),
    );
    patcher.add_change(
        &to_patch(health::DEF_SMALL),
        &to_offset(health_offset + entity::DEF_SMALL),
    );

    patcher.add_change(
        &to_patch(health::RED_SKULL_ENEMY),
        &to_offset(health_offset + entity::RED_SKULL_ENEMY),
    );
    patcher.add_change(
        &to_patch(health::BLUE_SKULL_ENEMY),
        &to_offset(health_offset + entity::BLUE_SKULL_ENEMY),
    );
}
