use crate::config::Config;
use crate::patcher::Patcher;

pub fn handle_qol_hacks(patcher: &mut Patcher, config: &Config) {
    if config.qol_hacks.faster_starting_fire {
        patcher.add_change("07", "087DE");
    }
    if config.qol_hacks.fix_hyper_laser {
        patcher.add_change("EAEAEA", "1FE2C");
    }
    if config.qol_hacks.enemy_erasers_unlocked_from_start {
        patcher.add_change("ff", "4206");
    }
    if config.qol_hacks.remove_flash {
        //instead of patching the calls i could just make the function not set anything, but this is already done
        patcher.add_change("0f", "18bbd"); //ee flash
        patcher.add_change("0f", "894c"); //end flash
        patcher.add_change("EAEAEA", "d375"); //boss flash
    }
}

