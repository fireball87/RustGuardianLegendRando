use crate::patcher::Patcher;
use im::*;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Clone)]
struct Boss {
    #[allow(dead_code)]
    boss: &'static str,
    patch: Vector<&'static str>,
    pointer: Vector<&'static str>,
    ids: Vector<&'static str>,
}

#[derive(Clone)]
pub struct OutputBoss {
    pub id: u32,
    pub pointer: u32,
}
pub fn randomize_bosses(
    patcher: &mut Patcher,
    randomize_final_boss: bool,
) -> (Vector<OutputBoss>, Vector<OutputBoss>, Option<OutputBoss>) {
    let mut table = vector![
        Boss {
            boss: "eyegore",
            patch: vector!["003780013A800242DA834FF6", "00378001028E0242DA834FF6"],
            pointer: vector!["C2BF", "CEBF"],
            ids: vector!["18", "19"],
        },
        Boss {
            boss: "fleepa",
            patch: vector!["0037800139888243B2"],
            pointer: vector!["B8B8"],
            ids: vector!["40", "41"],
        },
        Boss {
            boss: "optomon",
            patch: vector!["0037800140C68253F6"],
            pointer: vector!["5DA0"],
            ids: vector!["2d", "2e", "2f"],
        },
        Boss {
            boss: "crawdaddy",
            patch: vector!["0037808149BA"],
            pointer: vector!["FD9E"],
            ids: vector!["45"],
        },
        Boss {
            boss: "bombarderclawbot",
            patch: vector!["003780013588022AD88351F6"],
            pointer: vector!["F2BC"],
            ids: vector!["4c", "4d", "24", "25", "26"],
        },
        Boss {
            boss: "teramute",
            patch: vector!["0037800144B08203F2"],
            pointer: vector!["DBA3"],
            ids: vector!["46"],
        },
        Boss {
            boss: "glider",
            patch: vector!["0037800134C68235E6"],
            pointer: vector!["20AE"],
            ids: vector!["47"],
        },
        Boss {
            boss: "zibzub",
            patch: vector!["0037800135888241D2"],
            pointer: vector!["7CA7"],
            ids: vector!["23"],
        },
        Boss {
            boss: "grimgrin",
            patch: vector!["00378001049A821CA2"],
            pointer: vector!["8AB6"],
            ids: vector!["48", "49"],
        },
    ];

    if randomize_final_boss {
        table.push_back(Boss {
            boss: "it",
            patch: vector!["00378001368802459C8318D0"],
            pointer: vector!["0585"],
            ids: vector!["4f"],
        });
    }

    //patch graphics
    for boss in &table {
        for x in 0..boss.patch.len() {
            let bankstart: u32 = 0x8010;
            let flipped_pointer = format!("{}{}", &boss.pointer[x][2..], &boss.pointer[x][0..2]);
            let offset = format!(
                "{:X}",
                bankstart + u32::from_str_radix(&flipped_pointer, 16).unwrap()
            );
            patcher.add_change(&boss.patch[x], offset.as_str());
        }
    }

    let mut final_boss = None;

    if randomize_final_boss {
        let boss_array_id = rand::random::<usize>() % table.len();
        let id_array_id = rand::random::<usize>() % table[boss_array_id].ids.len();
        let id = table[boss_array_id].ids[id_array_id];

        println!("final boss is {}, {}", table[boss_array_id].boss, id);

        let pointer = if table[boss_array_id].pointer.len() > 1 {
            let pointer = table[boss_array_id].pointer[id_array_id];
            table[boss_array_id].pointer.remove(id_array_id);
            pointer
        } else {
            table[boss_array_id].pointer[0]
        };

        table[boss_array_id].ids.remove(id_array_id);
        if table[boss_array_id].ids.is_empty() {
            table.remove(boss_array_id);
        }

        final_boss = Some(OutputBoss {
            id: u32::from_str_radix(id, 16).unwrap(),
            pointer: u32::from_str_radix(pointer, 16).unwrap(),
        });
    }

    let mut c21bosses = Vector::new();
    let mut c21sourcelist = table.clone();
    for _ in 0..6 {
        let x = rand::thread_rng().gen_range(0..(c21sourcelist.len()));

        let key = rand::thread_rng().gen_range(0..(c21sourcelist[x].ids.len()));

        let pointer = if c21sourcelist[x].pointer.len() > 1 {
            c21sourcelist[x].pointer[key]
        } else {
            c21sourcelist[x].pointer[0]
        };
        let id = c21sourcelist[x].ids[key];
        c21bosses.push_back(OutputBoss {
            id: u32::from_str_radix(id, 16).unwrap(),
            pointer: u32::from_str_radix(pointer, 16).unwrap(),
        });
        c21sourcelist.remove(x);
    }

    let mut level_bosses = Vec::new();

    for boss in &table {
        for (key, id) in boss.ids.iter().enumerate() {
            let pointer = if boss.pointer.len() > 1 {
                boss.pointer[key]
            } else {
                boss.pointer[0]
            };
            level_bosses.push(OutputBoss {
                id: u32::from_str_radix(id, 16).unwrap(),
                pointer: u32::from_str_radix(pointer, 16).unwrap(),
            });
        }
    }

    level_bosses.shuffle(&mut rand::thread_rng());

    let eyeCluster = OutputBoss {
        id: u32::MAX,
        pointer: u32::MAX,
    };
    level_bosses.insert(6, eyeCluster.clone());
    level_bosses.insert(16, eyeCluster.clone());

    let return_bosses = Vector::from(level_bosses);

    for (index, row) in c21bosses.iter().enumerate() {
        println!("c21 boss {} is {}", index, row.id);
    }
    for (index, row) in return_bosses.iter().enumerate() {
        println!("corridor boss {} is {}", index, row.id);
    }
    (return_bosses, c21bosses, final_boss)
}
