pub fn randomize_minibosses(patcher: &mut Patcher, allow_missingno: bool, rng: &mut ChaCha8Rng) {
    // 22 minibosses
    // 3 * a number between 0 and 11, 12 if allow_missingno
    let monstervalues = if allow_missingno { 12 } else { 11 };

    let mut datum = String::new();
    for _ in 0..22 {
        let value = rng.gen_range(0..monstervalues * 3);
        datum.push_str(&format!("{:X}", value));
    }

    patcher.add_change(&datum, "1669D");
}
