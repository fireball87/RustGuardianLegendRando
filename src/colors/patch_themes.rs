use rand::Rng;
use rand_chacha::ChaCha8Rng;
use crate::config::Config;
use crate::patcher::Patcher;

struct Patch {
    address: &'static str,
    hex_code: &'static str
}
pub fn patch_all(cfg: &Config, patcher: &mut Patcher, rng: &mut ChaCha8Rng){



    let areas = [get_c0(), get_a0(), get_a1(), get_a3(), get_a5(), get_a7(), get_a9()];


    for area in areas {
        let index = rng.gen_range(0..area.len());
        let selected = &area[index];
        for values in selected {
            if(cfg.bad_ideas.completely_random_colors)
            {
                patcher.add_change(&*get_ran_palette(rng), values.address);
            }
            else {
                patcher.add_change(values.hex_code, values.address);
            }
        }
    }

}

fn get_c0() -> Vec<Vec<Patch>>{
    //patch c0
    // 0x17344 -> 1c,0c,16
    // used by background floor in c0
    // 0x1733E -> 11,01,16
    // used by foreground floor in c0
    // 0x17341 -> 22,12,16
    // used by eye cannons and stuff
    vec![
        //vanilla
        /*[
            Patch{address: "17344", hex_code: "1C0C16"},
            Patch{address: "1733E", hex_code: "110116"},
            Patch{address: "17341", hex_code: "221216"}

        ],*/
        //purple
        vec![
            Patch{address: "17344", hex_code: "140326"},
            Patch{address: "1733E", hex_code: "241326"},
            Patch{address: "17341", hex_code: "130307"}
        ],
        //grayscale
        vec![
            Patch{address: "17344", hex_code: "3D2D27"},
            Patch{address: "1733E", hex_code: "3D2D27"},
            Patch{address: "17341", hex_code: "2D3D27"}
        ],
        //icy
        vec![
            Patch{address: "1733E", hex_code: "3C2C32"},
            Patch{address: "17341", hex_code: "1C3C22"},
            Patch{address: "17344", hex_code: "1C0C31"},
        ],
        //burnt
        vec![
            Patch{address: "1733E", hex_code: "261707"},
            Patch{address: "17341", hex_code: "061628"},
            Patch{address: "17344", hex_code: "070818"},
        ],
    ]
}



fn get_a0() -> Vec<Vec<Patch>>{
    vec![
        //vanilla
        /*vec![
            Patch{address: "17326", hex_code: "301A0A"},
            Patch{address: "1732F", hex_code: "301000"},
        ],*/
        //bluey
        vec![
            Patch{address: "17326", hex_code: "301A0A"},
            Patch{address: "1732F", hex_code: "301000"},
        ],
        //sand
        vec![
            Patch{address: "17326", hex_code: "362838"},
            Patch{address: "1732F", hex_code: "362717"},
        ],
        //barbie
        vec![
            Patch{address: "17326", hex_code: "241504"},
            Patch{address: "1732F", hex_code: "243424"},
        ],
        //dank
        vec![
            Patch{address: "17326", hex_code: "0A1D08"},
            Patch{address: "1732F", hex_code: "0A0B0C"},
        ],
    ]
}

fn get_a3() -> Vec<Vec<Patch>>{
    vec![
        //vanilla
        /*vec![
            //1. used for flowers, 
            // first 2 colors should match 2
            // Final value should match final value of 3
            Patch{address: "1735C", hex_code: "30160A"},
            //2. demon plant, 
            // first 2 colors should match 1
            // last color should be close
            Patch{address: "1735F", hex_code: "301606"},
            //3. background
            Patch{address: "17362", hex_code: "2A1A0A"},
        ],*/
        //fall
        vec![
            //1. used for flowers, 
            // first 2 colors should match 2
            // Final value should match final value of 3
            Patch{address: "1735C", hex_code: "281707"},
            //2. demon plant, 
            // first 2 colors should match 1
            // last color should be close
            Patch{address: "1735F", hex_code: "271706"},
            //3. background
            Patch{address: "17362", hex_code: "261707"},
        ],
    ]
}


fn get_a9() -> Vec<Vec<Patch>>{
    vec![
        //vanilla
        vec![
            //1. donno 
            Patch{address: "1734A", hex_code: "271707"},
            //2. donno
            Patch{address: "1734D", hex_code: "37170F"},
            //3. background
            Patch{address: "17350", hex_code: "281808"},
        ],
    ]
}

fn get_a7() -> Vec<Vec<Patch>>{
    vec![
        //vanilla
        vec![
            //1. eyeballs seem to be it 
            Patch{address: "17353", hex_code: "302112"},
            //2. i think this is the blue plants
            Patch{address: "17356", hex_code: "221205"},
            //3. background
            Patch{address: "17359", hex_code: "250605"},
        ],
    ]
}

fn get_a5() -> Vec<Vec<Patch>>{
    vec![
        //vanilla
        vec![
            //1. blue orb background 
            Patch{address: "1736E", hex_code: "311202"},
            //2. volcano center
            Patch{address: "17371", hex_code: "351505"},
            //3. background
            Patch{address: "17374", hex_code: "301000"},
            // the floor hexes in the overworld are shared with
            // the crates for some reason, we still load volcano color
            // does something use that?
        ],
    ]
}


fn get_a1() -> Vec<Vec<Patch>>{
    vec![
        //vanilla
        /*vec![
            //1. used for grass, middle almost certainly must match
            Patch{address: "17365", hex_code: "2A1C1A"},
            //2. firing ground plants, middle color should match ground middle
            //last color is used for the center of the bubble spawners
            Patch{address: "17368", hex_code: "301C16"},
            //3. ground colors, middle should match 2 middle
            Patch{address: "1736B", hex_code: "2C1C0C"},
        ],*/
        //deep blue
        vec![
            //1. used for grass, middle almost certainly must match
            Patch{address: "17365", hex_code: "180104"},
            //2. firing ground plants, middle color should match ground middle
            //last color is used for the center of the bubble spawners
            Patch{address: "17368", hex_code: "140127"},
            //3. ground colors, middle should match 2 middle
            Patch{address: "1736B", hex_code: "110102"},
        ],
    ]
}

fn get_ran_palette(rng: &mut ChaCha8Rng) -> String{
    let colors = 
        vec![
            "01","02","03","04","05","06","07","08","09","0A","0B","0C",
            "11","12","13","14","15","16","17","18","19","1A","1B","1C",
            "21","22","23","24","25","26","27","28","29","2A","2B","2C",
            "31","32","33","34","35","36","37","38","39","3A","3B","3C",
            "1D","2D","3D","30"
        ];
    let c1 = rng.gen_range(0..colors.len());
    let c2 = rng.gen_range(0..colors.len());
    let c3 = rng.gen_range(0..colors.len());

    format!("{}{}{}",c1,c2,c3)

}