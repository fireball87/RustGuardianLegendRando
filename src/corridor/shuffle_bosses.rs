use crate::patcher::Patcher;
use rand::seq::SliceRandom;
use rand::Rng;
use rand_chacha::ChaCha8Rng;

#[derive(Clone)]
struct Boss {
    #[allow(dead_code)]
    boss: &'static str,
    patch: Vec<&'static str>,
    pointer: Vec<&'static str>,
    ids: Vec<&'static str>,
}

#[derive(Clone)]
pub struct OutputBoss {
    pub id: u32,
    pub pointer: u32,
}
pub fn randomize_bosses(
    patcher: &mut Patcher,
    randomize_final_boss: bool,
    rng: &mut ChaCha8Rng,
) -> (Vec<OutputBoss>, Vec<OutputBoss>, Option<OutputBoss>) {
    let mut table = vec![
        Boss {
            boss: "eyegore",
            patch: vec!["003780013A800242DA834FF6", "00378001028E0242DA834FF6"],
            pointer: vec!["C2BF", "CEBF"],
            ids: vec!["18", "19"],
        },
        Boss {
            boss: "fleepa",
            patch: vec!["0037800139888243B2"],
            pointer: vec!["B8B8"],
            ids: vec!["40", "41"],
        },
        Boss {
            boss: "optomon",
            patch: vec!["0037800140C68253F6"],
            pointer: vec!["5DA0"],
            ids: vec!["2d", "2e", "2f"],
        },
        Boss {
            boss: "crawdaddy",
            patch: vec!["0037808149BA"],
            pointer: vec!["FD9E"],
            ids: vec!["45"],
        },
        Boss {
            boss: "bombarderclawbot",
            patch: vec!["003780013588022AD88351F6"],
            pointer: vec!["F2BC"],
            ids: vec!["4c", "4d", "24", "25", "26"],
        },
        Boss {
            boss: "teramute",
            patch: vec!["0037800144B08203F2"],
            pointer: vec!["DBA3"],
            ids: vec!["46"],
        },
        Boss {
            boss: "glider",
            patch: vec!["0037800134C68235E6"],
            pointer: vec!["20AE"],
            ids: vec!["47"],
        },
        Boss {
            boss: "zibzub",
            patch: vec!["0037800135888241D2"],
            pointer: vec!["7CA7"],
            ids: vec!["23"],
        },
        Boss {
            boss: "grimgrin",
            patch: vec!["00378001049A821CA2"],
            pointer: vec!["8AB6"],
            ids: vec!["48", "49"],
        },
    ];

    if randomize_final_boss {
        table.push(Boss {
            boss: "it",
            patch: vec!["00378001368802459C8318D0"],
            pointer: vec!["0585"],
            ids: vec!["4f"],
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
            patcher.add_change(boss.patch[x], offset.as_str());
        }
    }

    let mut final_boss = None;

    if randomize_final_boss {
        let boss_array_id = rng.gen_range(0..table.len());
        let id_array_id = rng.gen_range(0..table[boss_array_id].ids.len());
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

        if id != "4f" {
            //fix it's graphics for shuffling
            patcher.add_change("00070F0E1935393400070E0D1F3B373B72797C7CFEE6E6ED7D7E7F7FCFDFDFFEFE8E7880000E32D8FEF0F880000E3CE0408080A6B9D74F4FB07060667F7BFFFF110010113E4EB86040CEB2D83E72F8E0C0CEBCE000070F0E1935393400070E0D1F3B373B7A5D4476F2E6EEFF7D7E7F7FCFDFDFFEC08E7FFEC39FF2DFFFF1FCFFFFFFFDE0EFDFB77F0DEFEDB6FB777F9FFFFFF7FF3DBAE97BC69C39E1FEFDF7873F7FFFFFFF7B319ACD6B663CFBF7FEFDFBF7FFFFFE8E7FFEC39FF2C7FEF1FCFFFFFFFDF81E60CF7F71F74FDFE19F3FBEBE7BFF7F7F3D0C1C26337B2B7F3F0B0B393C7C3D3B3F7D72377FFCAE3F3E7E7F7F7FFFFFD7FBFFF97FEFFFFFBEBFFFEFEE7FF7F5EFDFB77F0DEFED36FB777F9FFFFFF7FF3DFAE97BC79F3FFDFEFDF7873F7FFFFFEFDFB77D0CECEEB3FB777F9FFFFFF7FFEACB972895BCDFFF1F3F7FFFFAF3E8C0B9F3F3B7F3FBDDD7C78F8FCFCFE77B7E3F3F0F06370C0A062F2F173B3E0F0D070F1B3B674F5D371F0F172F5F7F7F3F1FBFBFF7F775DD9FBFFFFF7F7FFFFFFEFEDA9BF96161418100FBFBF9E1E1C18100555ABEF97FEFE9EABEBFFFEEEE7FF7F5FDF5CDEDEEEFBBB5F7FEFEDE9FDFFFFB87F01F6DEDE58FB7F8FFFF9F9FDFFFFFEFFE773FB9AADAFAFFFFBFFFDFDDFDFDB0FCE84A1E7EF4E4F0BCFC7E7EFEFCFCD46C58D8D8FC7E16EC7C78F8B8FC7E1EF475652A361E0B0FFB7A7A3F3F1F0F0D030311000603020006FF017f02112FAB5AEEE711ff0611FFFBC1FFD65D320E0000FF7FEF663F0F1100121100000303030F0F0E00000303040E030D03060D091F3F2F6003070F0F1F23317FDBB7AEF675BFBEE7E7CFDFCFEE7CFFE730FC68CA9E7EF4E4F03CF8FAFEFEFCFCD4ECD85858B4E6C6ECDCB8B8B87C7EFE061B3D3F3F77379F1F273B3F5F6FFF7FCF6676F67E4D6D7B3F5F6FFFFFBF9F9FE1E1E0C0C0C0E0E0E1E1E0C0C0C0E0E0E0F0D8C840000000E0F0F8F8780C06002ACB5A1212142C14FDFD7C1E1E1E3C1C1A161638081060C01E1A1A3010204080CCCAFA774B392A47FCBE9E5F7F776D04035000081E050000723030180E031100071180C0601100051180C0E0B0B868340A6AAB0570D8583C0E6E6F07","bad6");
            patcher.add_change("2c1c0c3d1707", "17380");
        }
    }

    let mut c21bosses = Vec::new();
    let mut c21sourcelist = table.clone();
    for _ in 0..6 {
        let x = rng.gen_range(0..(c21sourcelist.len()));

        let key = rng.gen_range(0..(c21sourcelist[x].ids.len()));

        let pointer = if c21sourcelist[x].pointer.len() > 1 {
            c21sourcelist[x].pointer[key]
        } else {
            c21sourcelist[x].pointer[0]
        };
        let id = c21sourcelist[x].ids[key];
        c21bosses.push(OutputBoss {
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

    level_bosses.shuffle(rng);

    let eye_cluster = OutputBoss {
        id: u32::MAX,
        pointer: u32::MAX,
    };
    level_bosses.insert(6, eye_cluster.clone());
    level_bosses.insert(16, eye_cluster.clone());

    //let return_bosses = Vec::from(level_bosses);

    for (index, row) in c21bosses.iter().enumerate() {
        println!("c21 boss {} is {:02X}", index, row.id);
    }
    for (index, row) in level_bosses.iter().enumerate() {
        println!("corridor boss {} is {:02X}", index + 1, row.id);
    }
    (level_bosses, c21bosses, final_boss)
}
