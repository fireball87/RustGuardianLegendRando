use crate::corridor::shuffle_bosses::OutputBoss;
use crate::patcher::Patcher;
use crate::tgl_error::{tgl_error, TGLError};
use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;

pub fn shuffle_corridors(
    patcher: &mut Patcher,
    shuffle_corridors: bool,
    shuffled_bosses: &Option<(Vec<OutputBoss>, Vec<OutputBoss>, Option<OutputBoss>)>,
    log: bool,
    rng: &mut ChaCha8Rng,
) -> Result<(), TGLError> {
    let mut table: Vec<[u32; 4]> = vec![
        [1, 0x40, 0x509C, 0x21],
        [2, 0x45, 0x7F9D, 0x21],
        [3, 0x2E, 0x5EA2, 0x22],
        [4, 0x46, 0x0CA4, 0x22],
        [5, 0x23, 0x8EA9, 0x23],
        [6, 0x47, 0x11AB, 0x23],
        [7, 0x06, 0x11B0, 0x24],
        [8, 0x49, 0x96B1, 0x24],
        [9, 0x18, 0xBFB6, 0x26],
        [10, 0x19, 0xC1B8, 0x26],
        [11, 0x2D, 0x2B9F, 0x21],
        [12, 0x41, 0x8EA0, 0x21],
        [13, 0x4C, 0xF7A5, 0x22],
        [14, 0x25, 0xB1A7, 0x22],
        [15, 0x24, 0x90AC, 0x23],
        [16, 0x2F, 0x29AE, 0x23],
        [17, 0x06, 0x46B3, 0x24],
        [18, 0x48, 0xE0B4, 0x24],
        [19, 0x4D, 0xFFBA, 0x25],
        [20, 0x26, 0x32BD, 0x25],
    ];
    // Refresh the list with the shuffled bosses

    let mut final_id = None;
    let mut c21_final_id = None;
    if let Some(shuffled_bosses) = shuffled_bosses {
        for (x, row) in table.iter_mut().enumerate() {
            if let Some(entry) = &shuffled_bosses.0.get(x) {
                if entry.id != u32::MAX {
                    row[1] = entry.id;
                }
            }
        }

        if let Some(c21_final) = &shuffled_bosses.1.last() {
            let id = c21_final.id;
            c21_final_id = Some(id);
            let boss_str = &format!("{:02X}", id);
            //bosses.push_str(boss_str);
            patcher.add_change(boss_str, "d3cb");
            patcher.add_change(boss_str, "d3ac");
        }

        if let Some(final_boss) = &shuffled_bosses.2 {
            final_id = Some(final_boss.id);
        }
    }
    if shuffle_corridors {
        table.shuffle(rng);
    }

    let mut bosses = String::new();
    let mut pointers = String::new();
    let mut graphics = String::new();
    if log {
        println!();
    }

    for row in table {
        print!("{:02X},", row[1]);

        bosses.push_str(&format!("{:02X}", row[1]));
        pointers.push_str(&format!("{:04X}", row[2]));
        graphics.push_str(&format!("{:02X}", row[3]));
    }

    //place the final boss into the string
    if let Some(id) = final_id {
        // try to shove the boss string into the c21 slot
        bosses.push_str(&format!(
            "{:02X}",
            c21_final_id.ok_or(tgl_error("c21_final_id was null"))?
        ));
        bosses.push_str(&format!("{:02X}", id));
    }

    if log {
        println!();
    }

    patcher.add_change(&bosses, "d162");
    patcher.add_change(&pointers, "10029");
    patcher.add_change(&graphics, "1ef66");

    Ok(())

    //change the boss tied to the corridor
    //boss table is d161 and starts at 0, we're not shifting 0 for the moment
    //boss id's for area //40,45,2E,46,23,47,06,49,18,19,2D,41,4C,25,24,2F,06,48,4D,26,4D,4F

    //change the corridor label itself
    //area table starts at 10027
    //area id's
    //10027    45 80    0
    //10029    50 9C    1
    //1002B    7F 9D    2
    //1002D    5E A2    3
    //1002F    0C A4    4
    //10031    8E A9    5
    //10033    11 AB    6
    //10035    11 B0    7
    //10037    96 B1    8
    //10039    BF B6    9
    //1003B    C1 B8    10
    //1003D    2B 9F    11
    //1003F    8E A0    12
    //10041    F7 A5    13
    //10043    B1 A7    14
    //10045    90 AC    15
    //10047    29 AE    16
    //10049    46 B3    17
    //1004B    E0 B4    18
    //1004D    FF BA    19
    //1004F    32 BD    20

    //need to do 3 things
    //change the graphics tied to the corridor
    //1ef65 is the start of the graphics id's
    //21,21,22,22,23,23,24,24,26,26,21,21,22,22,23,23,24,24,25,25
}
