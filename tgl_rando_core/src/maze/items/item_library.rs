use crate::tgl_error::TGLError;
use rand::Rng;
use rand_chacha::ChaCha8Rng;

pub fn get_item_blocks() -> Vec<&'static str> {
    vec!["AE94", "C994", "BE94", "B394"]
}

pub fn get_text_block(area: i32) -> Result<Vec<&'static str>, TGLError> {
    match area {
        0 => Ok(vec!["01", "02", "03", "10", "12"]), //removed "00" because we'll place it manually
        1 | 6 | 8 => Ok(vec![]),
        2 => Ok(vec!["0C"]),
        3 => Ok(vec!["0D"]),
        4 => Ok(vec!["0E"]),
        5 => Ok(vec!["0F"]),
        7 => Ok(vec!["11"]),
        9 => Ok(vec!["13"]),
        10 => Ok(vec!["14"]),
        _ => Err("Requested Invalid Area".into()),
    }
}

//transitionType is 0 for none, 1 for area, 2 for corridor
//direction is 1up, 2down, 3left, 4right
pub(crate) fn get_random_room_block(
    has_chips: bool,
    transition_type: i32,
    direction: i32,
    rng: &mut ChaCha8Rng,
) -> Result<String, TGLError> {
    match (has_chips, transition_type, direction) {
        (false, 0, _) => {
            //	6894 up arrow complete
            //	3d94 left arrow smashed
            //	4694 left arrow with red tail
            //	5094 right arrow smashed
            //	8794 down arrow smashed
            //	3f94 left arrow smashed
            //	8c94 down arrow smashed into a checkerboard pattern
            //	5d94 right arrow with a bunch of red blocks added in
            //	5294 right arrow with a smashed shadow
            //	e394 smashed block in an odd offset grid thing
            //	dd94 random block corners going away from eachother
            //	df94 a bunch of 2x2 squares
            //	d994 a bunch of u's in a cycle
            //	d194 red block us in a rotation
            //	d794 a bunch of 2x2 squares but now red
            //	d594 pointy small triangles but now in red blocks
            let values = [
                "6894", "3d94", "4694", "5094", "8794", "3f94", "8c94", "5d94", "5294", "e394",
                "dd94", "df94", "d994", "d194", "d794", "d594",
            ];
            Ok(values[rng.gen_range(0..values.len())].to_string())
        }
        (true, 0, _) => {
            //	6f94 up arrow with back smashed and 2 unrevealed chips
            //	4194 right arrow with 2 blue chips cut out
            //	9d94 arrow pointing down with a bunch of unrevealed chips in it
            let values = ["6f94", "4194", "9d94"];
            Ok(values[rng.gen_range(0..values.len())].to_string())
        }
        (false, 1, 1) => Ok("1c95".to_string()), //	1c95 a0 area transition going up with a bunch of extra red blocks
        (false, 1, 2) => {
            //	3c95 A0 area transition going down with 5 red blocks above
            //	4995 an area transition going down with 4 cut off blocks
            let values = ["3c95", "4995"];
            Ok(values[rng.gen_range(0..values.len())].to_string())
        }
        (false, 1, 3) => {
            //	e994 a0 area transition pattern going left
            //	f394 area transition going left with an upsidedown L cut out of it
            //	eb94 area transition going left with some blocks that are red
            let values = ["e994", "f394", "eb94"];
            Ok(values[rng.gen_range(0..values.len())].to_string())
        }
        (false, 1, 4) => Ok("0895".to_string()), //	0895 a0 area transition going right with 2 extra red blocks
        (true, 1, 1) => {
            //	1795 area transition up with some chips to pull up
            //	2995 area transition up with 2 blue chips and some random holes
            //	2495 area transition up with a chip block in the center of the room
            let values = ["1795", "2995", "2495"];
            Ok(values[rng.gen_range(0..values.len())].to_string())
        }
        (true, 1, 2) => {
            //	3795 area transition down with a chip block in the middle
            //	4495 area transition down with a blue chip in the middle
            let values = ["4495", "3795"];
            Ok(values[rng.gen_range(0..values.len())].to_string())
        }
        (true, 1, 3) => Ok("fb94".to_string()), //	fb94 area transition left with 4 chip tiles
        (true, 1, 4) => {
            //	0395 area transition right with some chips to pull up
            //	0d95 area transition to the right with 4 blue chips in the middle
            let values = ["0395", "0d95"];
            Ok(values[rng.gen_range(0..values.len())].to_string())
        }
        (false, 2, 1) => {
            //	7a95 corridor topper with up exit 6 point
            //	8595 corridor topper up with 8 point
            let values = ["7a95", "8595"];
            Ok(values[rng.gen_range(0..values.len())].to_string())
        }
        (false, 2, 2) => {
            //	9095 corridor topper with down exit 6 point
            //	9B95 corridor topper with down exit 8 point
            let values = ["9095", "9B95"];
            Ok(values[rng.gen_range(0..values.len())].to_string())
        }
        (false, 2, 3) => {
            //	4e95 corridor topper with left exit 6 point
            //	5995 corridor topper left with 8 4oint
            let values = ["4e95", "5995"];
            Ok(values[rng.gen_range(0..values.len())].to_string())
        }
        (false, 2, 4) => {
            //	6495 corridor topper with right exit 6 point
            //	6f95 corridor topper with right exit 8 point
            let values = ["6495", "6f95"];
            Ok(values[rng.gen_range(0..values.len())].to_string())
        }
        (true, 2, _) => {
            Err("tried to place a chip decoration on a corridor, the game has none of those".into())
        }
        _ => Err("somehow didn't hit an if block on the decoration placement".into()),
    }
}

pub(crate) fn get_p_chip_room() -> String {
    //ea95 the p chip rooms
    "ea95".to_string()
}

pub(crate) fn get_cardinal_letter(letter: &str) -> Result<String, TGLError> {
    match letter.to_ascii_uppercase().as_str() {
        "N" => Ok("a695".to_string()),
        "S" => Ok("b495".to_string()),
        "E" => Ok("c895".to_string()),
        "W" => Ok("d995".to_string()),
        _ => Err("don't request bad cardinal letters...".into()),
    }
}

pub fn get_miniboss(area: i32) -> Result<Vec<&'static str>, TGLError> {
    match area {
        0 => Ok(vec!["0B", "0C"]),
        1 => Ok(vec!["0D", "0E"]),
        2 => Ok(vec!["0F", "10"]),
        3 => Ok(vec!["12", "11"]),
        4 => Ok(vec!["13", "14"]),
        5 => Ok(vec!["15", "16"]),
        6 => Ok(vec!["17", "18"]),
        7 => Ok(vec!["19", "1A"]),
        8 => Ok(vec!["1B", "1C"]),
        9 => Ok(vec!["1D", "1E"]),
        10 => Ok(vec!["1F", "20"]),
        _ => Err("Requested Invalid Area".into()),
    }
}

// pub fn get_multi_shop_library() -> Vec<Vec<&'static str>> {
//     vec![
//         vec![],
//         vec![],
//         vec!["3F"],
//         vec![],
//         vec!["41"],
//         vec![],
//         vec!["42"],
//         vec!["40"],
//         vec![],
//         vec![],
//         vec!["43"],
//     ]
// }

// pub fn get_single_shop_library() -> Vec<Vec<&'static str>> {
//     vec![
//         vec!["3D", "3E", "3C", "3B", "3A"],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//     ]
// }

// pub fn get_item_library() -> Vec<Vec<&'static str>> {
//     vec![
//         vec!["21", "05", "39", "35"],
//         vec!["24", "23", "26"],
//         vec!["36", "25"],
//         vec!["28", "27"],
//         vec!["29", "2A", "37"],
//         vec!["01", "2B", "2C"],
//         vec!["0A", "2D", "2E", "02"],
//         vec!["03", "2F", "30"],
//         vec!["31", "38", "04", "32"],
//         vec!["06", "33", "34", "08", "07"],
//         vec!["09"],
//     ]
// }

// fn get_secret_items(area: i32) -> Vec<&'static str> {
//     match area {
//         0 => vec!["35", "05", "00"],
//         1 => vec!["39", "2b"],
//         2 => vec!["07", "04", "38"],
//         3 => vec!["32", "25", "24"],
//         4 => vec!["09", "36"],
//         5 => vec!["01", "23", "30"],
//         6 => vec!["02", "34"],
//         7 => vec!["2a", "06", "0a"],
//         8 => vec!["2c", "33"],
//         9 => vec!["2e", "26"],
//         10 => vec!["22"],
//         _ => panic!("Requested Invalid Area"),
//     }
// }
//
// pub(crate) fn get_secret_text_block(area: i32) -> Vec<&'static str> {
//     match area {
//         0 => vec!["01", "02", "03"], //removed "00" because we'll place it manually earlier
//         1 => vec![],
//         2 => vec!["0f"],
//         3 => vec!["10"],
//         4 => vec![], //text 12 will be generated with c4
//         5 => vec!["13"],
//         6 => vec!["0e"],
//         7 => vec!["0c"],
//         8 => vec!["0d"],
//         9 => vec!["14"],
//         10 => vec!["11"],
//         _ => panic!("Requested Invalid Area"),
//     }
// }
//
// fn get_secret_multi_shop(area: i32) -> Vec<&'static str> {
//     match area {
//         2 => vec!["3f"],
//         3 => vec!["40"],
//         5 => vec!["41"],
//         6 => vec!["42"],
//         10 => vec!["43"],
//         0 | 1 | 4 | 7 | 8 | 9 => vec![],
//         _ => panic!("Requested Invalid Area"),
//     }
// }
//
// fn get_secret_single_shop(area: i32) -> Vec<&'static str> {
//     match area {
//         0 => vec!["3d", "3a", "3b"],
//         2 => vec!["3c"],
//         4 => vec!["3e"],
//         1 | 3 | 5 | 6 | 7 | 8 | 9 | 10 => vec![],
//         _ => panic!("Requested Invalid Area"),
//     }
// }
