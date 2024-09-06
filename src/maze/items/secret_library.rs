fn get_items(area: i32) -> Vec<&'static str> {
    match area {
        0 => vec!["35", "05", "00"],
        1 => vec!["39", "2b"],
        2 => vec!["07", "04", "38"],
        3 => vec!["32", "25", "24"],
        4 => vec!["09", "36"],
        5 => vec!["01", "23", "30"],
        6 => vec!["02", "34"],
        7 => vec!["2a", "06", "0a"],
        8 => vec!["2c", "33"],
        9 => vec!["2e", "26"],
        10 => vec!["22"],
        _ => panic!("Requested Invalid Area"),
    }
}

pub(crate) fn get_text_block(area: i32) -> Vec<&'static str> {
    match area {
        0 => vec!["01", "02", "03"], //removed "00" because we'll place it manually earlier
        1 => vec![],
        2 => vec!["0f"],
        3 => vec!["10"],
        4 => vec![], //text 12 will be generated with c4
        5 => vec!["13"],
        6 => vec!["0e"],
        7 => vec!["0c"],
        8 => vec!["0d"],
        9 => vec!["14"],
        10 => vec!["11"],
        _ => panic!("Requested Invalid Area"),
    }
}

fn get_multi_shop(area: i32) -> Vec<&'static str> {
    match area {
        2 => vec!["3f"],
        3 => vec!["40"],
        5 => vec!["41"],
        6 => vec!["42"],
        10 => vec!["43"],
        0 | 1 | 4 | 7 | 8 | 9 => vec![],
        _ => panic!("Requested Invalid Area"),
    }
}

fn get_single_shop(area: i32) -> Vec<&'static str> {
    match area {
        0 => vec!["3d", "3a", "3b"],
        2 => vec!["3c"],
        4 => vec!["3e"],
        1 | 3 | 5 | 6 | 7 | 8 | 9 | 10 => vec![],
        _ => panic!("Requested Invalid Area"),
    }
}
