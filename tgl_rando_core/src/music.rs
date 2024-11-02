use crate::config::{Config, MusicOptions};
use crate::patcher::Patcher;
use rand::Rng;
use rand_chacha::ChaCha8Rng;

pub fn patch_music(patcher: &mut Patcher, config: &Config, rng: &mut ChaCha8Rng) {
    let area_tracks = vec![2, 7, 9, 13, 14];
    let corridor_tracks = vec![4, 5, 8, 12, 20, 30];
    let area_plus_corridor = [area_tracks.clone(), corridor_tracks.clone()].concat();

    const AREA_OFFSET: &str = "1ef26";
    const AREA_LENGTH: usize = 11;
    match config.music_config.area_music {
        MusicOptions::Untouched => {}
        MusicOptions::Corridor => {
            patch_music_set(patcher, rng, &corridor_tracks, AREA_OFFSET, AREA_LENGTH)
        }
        MusicOptions::Overworld => {
            patch_music_set(patcher, rng, &area_tracks, AREA_OFFSET, AREA_LENGTH)
        }
        MusicOptions::CorridorAndOverworld => {
            patch_music_set(patcher, rng, &area_plus_corridor, AREA_OFFSET, AREA_LENGTH)
        }
    }

    const CORRIDOR_OFFSET: &str = "1ef3a";
    const CORRIDOR_LENGTH: usize = 22;
    match config.music_config.corridor_music {
        MusicOptions::Untouched => {}
        MusicOptions::Corridor => patch_music_set(
            patcher,
            rng,
            &corridor_tracks,
            CORRIDOR_OFFSET,
            CORRIDOR_LENGTH,
        ),
        MusicOptions::Overworld => {
            patch_music_set(patcher, rng, &area_tracks, CORRIDOR_OFFSET, CORRIDOR_LENGTH)
        }
        MusicOptions::CorridorAndOverworld => patch_music_set(
            patcher,
            rng,
            &area_plus_corridor,
            CORRIDOR_OFFSET,
            CORRIDOR_LENGTH,
        ),
    }
}

fn patch_music_set(
    patcher: &mut Patcher,
    rng: &mut ChaCha8Rng,
    source_set: &[i32],
    address: &str,
    length: usize,
) {
    let mut output = String::new();
    for _ in 0..length {
        let index = rng.gen_range(0..source_set.len());
        let value = source_set[index];
        output.push_str(&format!("{:02X}", value));
    }

    patcher.add_change(&output, address);
}
/*tracks order

0x1ef26 areas -> 09 07 07 02 02 07 0D 07 02 0D 0E
only death -> 10
save rooms -> 18 18
text room -> 13
cleared corridor -> 10
what -> 00
uncleared corridor twice (is one the locked c21?)-> 18 18
didn't mark -> 12
0x1ef3a corridors -> 1E 15 15 04 04 05 05 0C 0C 08 08 15 15 04 04 05 05 0C 0C 08 08 1C
escape -> 19
ending -> 0F
this data ends somewhere at this point? 05 06 0F 08 01 08 0C 0C 0E 09 0C 10 0B 11 0F 07 0F 0F 0E

*/
/*audio in sound test
1 title screen
2 a 3, 4, 8
3 boss theme 1
4 c 3, 4, 13, 14
5 c 5, 6, 15, 16
6 boss theme 2
7 a 1 2 5 7
8 c 9 10 19 20
9 a0
10(0A) death
11(0B), 12 c7,8,17,18
13(0D), a 6,9
14(0E), a 10
15(0F), ending
16(10), cleared corridor
17(11), 18 uncleared corridor
19(13), text room
20(14), 21 c 1 2 11 12
22(16) boss theme 3
23(17) corridor victory
24(18) shop and save
25(19) password entry/escape

28(1c) c21

29(1d) a different corridor victory
30(1e), c0*/
