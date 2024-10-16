use crate::config::Config;
use crate::maze;
use crate::maze::items::item_generator::ItemLibrary;
use crate::maze::map::Map;
use crate::maze::room::RoomType;
use crate::maze::{area_division, items};
use crate::tgl_error::{tgl_error, TGLError};
use rand::prelude::SliceRandom;
use rand::Rng;
use rand_chacha::ChaCha8Rng;

pub struct Generator;

impl Generator {
    // The Concept map design process
    // create the map
    // subdivide the map into areas off a0 (trying to keep a tgl like grid map)
    // generate a multiple path maze for each area
    // find all candidates for connections between two touching areas (and select Some based on flags)
    // number All areas and assign progression with the note that areas that will connect need to be sequential
    // place rooms and items and stuff, something I can consider more after I do maze gen

    pub fn run(
        &self,
        item_library: &ItemLibrary,
        cfg: &Config,
        rng: &mut ChaCha8Rng,
    ) -> Result<Map, TGLError> {
        let map_cfg = &cfg.map_config;
        //create the map

        let mut map = Map::new();

        //subdivide the map into areas off a0 (trying to keep a tgl like grid map)
        self.subdivide_map(rng, &mut map)?;

        //we don't have this really, we need to subdivide then fill the map

        self.shuffle_areas(rng, &mut map)?;

        self.grow_a0_ring(&mut map)?; // i need to place cardinal directions before mapping the starting points, because the starting points will grow out a0 and break the calculation
        self.place_cardinal_directions(&mut map, rng)?;

        self.find_starting_points(&mut map, rng)?;

        for i in 1..=10 {
            self.grow_zone(
                i,
                rng.gen_range(map_cfg.min_area_size..=map_cfg.max_area_size),
                &mut map,
                rng,
            )?;
            self.add_connections(&mut map, i, map_cfg.desired_connections, false, false, rng)?;
            self.add_connections(
                &mut map,
                i,
                map_cfg.desired_one_way_connections,
                true,
                map_cfg.portal_only_one_ways,
                rng,
            )?;
        }
        self.grow_zone(0, 50, &mut map, rng)?;
        self.add_connections(&mut map, 0, map_cfg.desired_connections, false, false, rng)?;
        self.add_connections(
            &mut map,
            0,
            map_cfg.desired_one_way_connections,
            true,
            map_cfg.portal_only_one_ways,
            rng,
        )?;

        self.placestarting_point_rooms(&mut map)?;
        self.place_area_decorations(&mut map, rng)?;

        self.place_starting_text_room(&mut map);

        // place all my items
        for i in 0..=10 {
            self.place_important_rooms(&mut map, item_library, i, rng)?;
            self.place_items_and_minibosses(&mut map, item_library, i, rng)?;
            self.place_non_important_rooms(&mut map, i, rng)?;
        }

        self.place_corridor_decorations(&mut map, rng)?;
        self.place_random_decorations(&mut map, map_cfg.decoration_odds, map_cfg.chip_odds, rng)?;

        self.populate_enemies(&mut map, map_cfg.empty_room_odds, rng);
        let bytes = self.count_all_room_bytes(&map);
        if cfg.log {
            println!("{}", bytes);
        }
        if bytes > 1916 {
            Err("Produced map that is too large".into())
        } else {
            Ok(map)
        }
    }

    fn count_all_room_bytes(&self, map: &Map) -> i32 {
        let mut total = 0;
        for y_pos in 0..24 {
            for x_pos in 0..24 {
                total += map.data[[y_pos, x_pos]].count_bytes();
            }
        }
        total
    }

    fn subdivide_map(&self, rng: &mut ChaCha8Rng, map: &mut Map) -> Result<(), TGLError> {
        let division = area_division::get_sub_division(rng)?;
        for i in 0..division.len() {
            for j in 0..division[0].len() {
                let data = &mut map.data[[i, j]];
                data.area = Some(division[i][j]);
            }
        }
        Ok(())
    }

    fn shuffle_areas(&self, rng: &mut ChaCha8Rng, map: &mut Map) -> Result<(), TGLError> {
        let mut newareas = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        newareas.shuffle(rng);

        for item in &mut map.data {
            let area = item.area.ok_or(tgl_error("empty area"))?;

            //for item in row {
            if (area >= 1) && (area <= 10) {
                item.area = Some(newareas[area as usize - 1]);
            }
            //}
        }
        Ok(())
    }

    fn find_starting_points(&self, map: &mut Map, rng: &mut ChaCha8Rng) -> Result<(), TGLError> {
        let mut possible_points: Vec<Vec<(usize, usize, &str)>> = vec![vec![]; 10];

        let err = "None value found in unwrapping of value in find_starting_points";
        for y_pos in 0..24 {
            for x_pos in 0..24 {
                if map.data[[y_pos, x_pos]].area.ok_or(tgl_error(err))? == -1 {
                    let above_block = if y_pos > 0 {
                        Some(&map.data[[y_pos - 1, x_pos]])
                    } else {
                        None
                    };
                    let below_block = if y_pos < 23 {
                        Some(&map.data[[y_pos + 1, x_pos]])
                    } else {
                        None
                    };
                    let left_block = if x_pos > 0 {
                        Some(&map.data[[y_pos, x_pos - 1]])
                    } else {
                        None
                    };
                    let right_block = if x_pos < 23 {
                        Some(&map.data[[y_pos, x_pos + 1]])
                    } else {
                        None
                    };

                    if let Some(above) = above_block {
                        if above.area.ok_or(tgl_error(err))? == 0 {
                            if let Some(below) = below_block {
                                if below.area.ok_or(tgl_error(err))? > 0
                                    && below.area.ok_or(tgl_error(err))? <= 10
                                {
                                    possible_points[below.area.ok_or(tgl_error(err))? as usize - 1]
                                        .push((y_pos, x_pos, "up"));
                                    continue;
                                }
                            }
                            continue;
                        }
                    }

                    if let Some(below) = below_block {
                        if below.area.ok_or(tgl_error(err))? == 0 {
                            if let Some(above) = above_block {
                                if above.area.ok_or(tgl_error(err))? > 0
                                    && above.area.ok_or(tgl_error(err))? <= 10
                                {
                                    possible_points[above.area.ok_or(tgl_error(err))? as usize - 1]
                                        .push((y_pos, x_pos, "down"));
                                    continue;
                                }
                            }
                            continue;
                        }
                    }

                    if let Some(left) = left_block {
                        if left.area.ok_or(tgl_error(err))? == 0 {
                            if let Some(right) = right_block {
                                if right.area.ok_or(tgl_error(err))? > 0
                                    && right.area.ok_or(tgl_error(err))? <= 10
                                {
                                    possible_points[right.area.ok_or(tgl_error(err))? as usize - 1]
                                        .push((y_pos, x_pos, "left"));
                                    continue;
                                }
                            }
                            continue;
                        }
                    }

                    if let Some(right) = right_block {
                        if right.area.ok_or(tgl_error(err))? == 0 {
                            if let Some(left) = left_block {
                                if left.area.ok_or(tgl_error(err))? > 0
                                    && left.area.ok_or(tgl_error(err))? <= 10
                                {
                                    possible_points[left.area.ok_or(tgl_error(err))? as usize - 1]
                                        .push((y_pos, x_pos, "right"));
                                    continue;
                                }
                            }
                            continue;
                        }
                    }
                }
            }
        }

        for (area, position_array) in possible_points.iter().enumerate() {
            if position_array.is_empty() {
                return Err(format!(
                    "Area {} did not generate with any possible entry points.",
                    area + 1
                )
                .into());
            }

            let index = rng.gen_range(0..position_array.len());
            let value = &position_array[index];

            let (y_pos, x_pos, dir) = value;

            map.data[[*y_pos, *x_pos]].area = Some(0);
            map.data[[*y_pos, *x_pos]].starting_point = true;
            map.data[[*y_pos, *x_pos]].accessible = true;

            match *dir {
                "down" => {
                    map.data[[*y_pos, *x_pos]].exit_down = true;
                    map.data[[*y_pos, *x_pos]].exit_up = true;

                    map.data[[*y_pos - 1, *x_pos]].exit_down = true;
                    map.data[[*y_pos - 1, *x_pos]].starting_point = true;
                    map.data[[*y_pos - 1, *x_pos]].accessible = true;

                    map.data[[*y_pos + 1, *x_pos]].exit_up = true;
                    map.data[[*y_pos + 1, *x_pos]].accessible = true;
                }
                "up" => {
                    map.data[[*y_pos, *x_pos]].exit_down = true;
                    map.data[[*y_pos, *x_pos]].exit_up = true;

                    map.data[[*y_pos + 1, *x_pos]].exit_up = true;
                    map.data[[*y_pos + 1, *x_pos]].starting_point = true;
                    map.data[[*y_pos + 1, *x_pos]].accessible = true;

                    map.data[[*y_pos - 1, *x_pos]].exit_down = true;
                    map.data[[*y_pos - 1, *x_pos]].accessible = true;
                }
                "right" => {
                    map.data[[*y_pos, *x_pos]].exit_left = true;
                    map.data[[*y_pos, *x_pos]].exit_right = true;

                    map.data[[*y_pos, *x_pos - 1]].exit_right = true;
                    map.data[[*y_pos, *x_pos - 1]].starting_point = true;
                    map.data[[*y_pos, *x_pos - 1]].accessible = true;

                    map.data[[*y_pos, *x_pos + 1]].exit_left = true;
                    map.data[[*y_pos, *x_pos + 1]].accessible = true;
                }
                "left" => {
                    map.data[[*y_pos, *x_pos]].exit_left = true;
                    map.data[[*y_pos, *x_pos]].exit_right = true;

                    map.data[[*y_pos, *x_pos + 1]].exit_left = true;
                    map.data[[*y_pos, *x_pos + 1]].starting_point = true;
                    map.data[[*y_pos, *x_pos + 1]].accessible = true;

                    map.data[[*y_pos, *x_pos - 1]].exit_right = true;
                    map.data[[*y_pos, *x_pos - 1]].accessible = true;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn grow_zone(
        &self,
        zone: i32,
        mut desired_size: usize,
        map: &mut Map,
        rng: &mut ChaCha8Rng,
    ) -> Result<(), TGLError> {
        let err = "area was None when a value was expected";

        let mut possible_grow_points = Vec::new();

        for y_pos in 0..24 {
            for x_pos in 0..24 {
                if map.data[[y_pos, x_pos]].area.ok_or(tgl_error(err))? == zone
                    && map.data[[y_pos, x_pos]].accessible
                {
                    possible_grow_points.push((y_pos, x_pos));
                }
            }
        }

        while desired_size > 0 && !possible_grow_points.is_empty() {
            let index = rng.gen_range(0..possible_grow_points.len());
            let (y_pos, x_pos) = possible_grow_points[index];

            let grow_direction = rng.gen_range(0..=3);
            let sequence = rng.gen_range(0..=5);

            let (up, left, right, down) = match sequence {
                0 => (0, 1, 2, 3),
                1 => (0, 1, 3, 2),
                2 => (0, 2, 3, 1),
                3 => (0, 2, 1, 3),
                4 => (0, 3, 1, 2),
                5 => (0, 3, 2, 1),
                _ => unreachable!(),
            };

            let mut grew = false;
            for i in 0..4 {
                let rem = (grow_direction + i) % 4;
                match rem {
                    d if d == up
                        && y_pos > 0
                        && map.data[[y_pos - 1, x_pos]].area.ok_or(tgl_error(err))? == zone
                        && !map.data[[y_pos - 1, x_pos]].accessible =>
                    {
                        map.data[[y_pos, x_pos]].exit_up = true;
                        map.data[[y_pos - 1, x_pos]].exit_down = true;
                        map.data[[y_pos - 1, x_pos]].accessible = true;
                        possible_grow_points.push((y_pos - 1, x_pos));
                        grew = true;
                    }
                    d if d == left
                        && x_pos > 0
                        && map.data[[y_pos, x_pos - 1]].area.ok_or(tgl_error(err))? == zone
                        && !map.data[[y_pos, x_pos - 1]].accessible =>
                    {
                        map.data[[y_pos, x_pos]].exit_left = true;
                        map.data[[y_pos, x_pos - 1]].exit_right = true;
                        map.data[[y_pos, x_pos - 1]].accessible = true;
                        possible_grow_points.push((y_pos, x_pos - 1));
                        grew = true;
                    }
                    d if d == right
                        && x_pos < 23
                        && map.data[[y_pos, x_pos + 1]].area.ok_or(tgl_error(err))? == zone
                        && !map.data[[y_pos, x_pos + 1]].accessible =>
                    {
                        map.data[[y_pos, x_pos]].exit_right = true;
                        map.data[[y_pos, x_pos + 1]].exit_left = true;
                        map.data[[y_pos, x_pos + 1]].accessible = true;
                        possible_grow_points.push((y_pos, x_pos + 1));
                        grew = true;
                    }
                    d if d == down
                        && y_pos < 23
                        && map.data[[y_pos + 1, x_pos]].area.ok_or(tgl_error(err))? == zone
                        && !map.data[[y_pos + 1, x_pos]].accessible =>
                    {
                        map.data[[y_pos, x_pos]].exit_down = true;
                        map.data[[y_pos + 1, x_pos]].exit_up = true;
                        map.data[[y_pos + 1, x_pos]].accessible = true;
                        possible_grow_points.push((y_pos + 1, x_pos));
                        grew = true;
                    }
                    _ => {}
                }
                if grew {
                    break;
                }
            }

            if !grew {
                possible_grow_points.remove(index);
            } else {
                desired_size -= 1;
            }
        }
        Ok(())
    }
    fn grow_a0_ring(&self, map: &mut Map) -> Result<(), TGLError> {
        let err = "area was None when a value was expected";

        // form the outside ring
        for y_pos in 0..24 {
            for x_pos in 0..24 {
                if (map.data[[y_pos,x_pos]].area.ok_or(tgl_error(err))? == 0)
                    //&& (map.data[[y_pos,x_pos]].avoid_special)
                    && (map.data[[y_pos,x_pos]].room_type == RoomType::Normal)
                //&& (map.data[[y_pos,x_pos]].accessible)
                {
                    let mut grow_vertically = false;
                    let mut grow_horizontally = false;

                    if y_pos > 0 && map.data[[y_pos - 1, x_pos]].area.ok_or(tgl_error(err))? <= -1
                        || y_pos < 23
                            && map.data[[y_pos + 1, x_pos]].area.ok_or(tgl_error(err))? <= -1
                    {
                        grow_horizontally = true;
                    }

                    if x_pos > 0 && map.data[[y_pos, x_pos - 1]].area.ok_or(tgl_error(err))? <= -1
                        || x_pos < 23
                            && map.data[[y_pos, x_pos + 1]].area.ok_or(tgl_error(err))? <= -1
                    {
                        grow_vertically = true;
                    }

                    if grow_vertically {
                        map.data[[y_pos, x_pos]].accessible = true;
                        map.data[[y_pos, x_pos]].avoid_special = true;
                        // grow up if can
                        if map.data[[y_pos - 1, x_pos]].area.ok_or(tgl_error(err))? == 0 {
                            map.data[[y_pos, x_pos]].exit_up = true;
                            map.data[[y_pos - 1, x_pos]].accessible = true;
                            map.data[[y_pos - 1, x_pos]].avoid_special = true;
                            map.data[[y_pos - 1, x_pos]].exit_down = true;
                        }
                        // grow down If can
                        if map.data[[y_pos + 1, x_pos]].area.ok_or(tgl_error(err))? == 0 {
                            map.data[[y_pos, x_pos]].exit_down = true;
                            map.data[[y_pos + 1, x_pos]].accessible = true;
                            map.data[[y_pos + 1, x_pos]].avoid_special = true;
                            map.data[[y_pos + 1, x_pos]].exit_up = true;
                        }
                    }

                    if grow_horizontally {
                        map.data[[y_pos, x_pos]].accessible = true;
                        map.data[[y_pos, x_pos]].avoid_special = true;

                        // grow left if can
                        if map.data[[y_pos, x_pos - 1]].area.ok_or(tgl_error(err))? == 0 {
                            map.data[[y_pos, x_pos]].exit_left = true;
                            map.data[[y_pos, x_pos - 1]].accessible = true;
                            map.data[[y_pos, x_pos - 1]].exit_right = true;
                            map.data[[y_pos, x_pos - 1]].avoid_special = true;
                        }
                        // grow right If can
                        if map.data[[y_pos, x_pos + 1]].area.ok_or(tgl_error(err))? == 0 {
                            map.data[[y_pos, x_pos]].exit_right = true;
                            map.data[[y_pos, x_pos + 1]].accessible = true;
                            map.data[[y_pos, x_pos + 1]].exit_left = true;
                            map.data[[y_pos, x_pos + 1]].avoid_special = true;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn place_items_and_minibosses(
        &self,
        map: &mut Map,
        item_library: &ItemLibrary,
        area: i32,
        rng: &mut ChaCha8Rng,
    ) -> Result<(), TGLError> {
        let mut locations = self.create_list_of_suitable_rooms(map, area, false, false)?;

        let items_to_place = item_library.item_library[area as usize].clone();

        // minibosses are the same between secret and the normal items

        let minibosses_to_place = maze::items::item_library::get_miniboss(area);

        let item_blocksets = maze::items::item_library::get_item_blocks();

        for item in &items_to_place {
            if !locations.is_empty() {
                let index = rng.gen_range(0..locations.len());

                let y_pos = &locations[index].0;
                let x_pos = &locations[index].1;
                map.data[[*y_pos, *x_pos]].room_type = RoomType::Item;
                map.data[[*y_pos, *x_pos]].item_id = Some(item.clone());
                map.data[[*y_pos, *x_pos]].block_set = Some(
                    item_blocksets
                        .get(rng.gen_range(0..item_blocksets.len()))
                        .ok_or(tgl_error("range failed in place items"))?
                        .to_string(),
                );
                locations.remove(index);
            } else {
                return Err("map has no valid spot to place an item".into());
            }
        }

        for item in &minibosses_to_place? {
            if !locations.is_empty() {
                let index = rng.gen_range(0..locations.len());

                let y_pos = &locations[index].0;
                let x_pos = &locations[index].1;
                map.data[[*y_pos, *x_pos]].room_type = RoomType::Miniboss;
                map.data[[*y_pos, *x_pos]].item_id = Some(item.to_string());

                locations.remove(index);
            } else {
                return Err("map has no valid spot to place a miniboss".into());
            }
        }
        Ok(())
    }

    fn place_starting_text_room(&self, map: &mut Map) {
        let y_pos = 12;
        let x_pos = 11;
        map.data[[y_pos, x_pos]].room_type = RoomType::Text;
        map.data[[y_pos, x_pos]].item_id = Some("00".to_string());
    }
    //
    fn place_important_rooms(
        &self,
        map: &mut Map,
        item_library: &ItemLibrary,
        area: i32,
        rng: &mut ChaCha8Rng,
    ) -> Result<(), TGLError> {
        let mut locations = self.create_list_of_suitable_rooms(map, area, true, false)?;

        // place corridors
        if area == 0 {
            self.place_corridor(21, &mut locations, map, rng)?;
        } else if area == 1 {
            self.place_corridor(11, &mut locations, map, rng)?;
        } else {
            self.place_corridor(area, &mut locations, map, rng)?;
            self.place_corridor(area + 10, &mut locations, map, rng)?;
        }

        // place single shops

        let singleshops = item_library.single_shop_library[area as usize].clone();
        let multishops = item_library.multi_shop_library[area as usize].clone();

        for item in &singleshops {
            if !locations.is_empty() {
                let index = rng.gen_range(0..locations.len());

                let y_pos = &locations[index].0;
                let x_pos = &locations[index].1;
                map.data[[*y_pos, *x_pos]].room_type = RoomType::SingleShop;
                map.data[[*y_pos, *x_pos]].item_id = Some(item.clone());

                locations.remove(index);
            } else {
                return Err("map has no valid spot to place a single_shop".into());
            }
        }

        // place multishops
        for item in &multishops {
            if !locations.is_empty() {
                let index = rng.gen_range(0..locations.len());

                let y_pos = &locations[index].0;
                let x_pos = &locations[index].1;
                map.data[[*y_pos, *x_pos]].room_type = RoomType::MultiShop;
                map.data[[*y_pos, *x_pos]].item_id = Some(item.clone());

                locations.remove(index);
            } else {
                return Err("map has no valid spot to place a multi_shop".into());
            }
        }
        Ok(())
    }

    fn place_non_important_rooms(
        &self,
        map: &mut Map,
        area: i32,
        rng: &mut ChaCha8Rng,
    ) -> Result<(), TGLError> {
        let mut locations = self.create_list_of_suitable_rooms(map, area, true, false)?;

        // place save room
        if area <= 1 {
            if !locations.is_empty() {
                let index = rng.gen_range(0..locations.len());

                let y_pos = &locations[index].0;
                let x_pos = &locations[index].1;
                map.data[[*y_pos, *x_pos]].room_type = RoomType::Save;

                locations.remove(index);
            } else {
                return Err("map has no valid spot to place a save room".into());
            }
        }

        // place text rooms
        let textrooms = items::item_library::get_text_block(area)?;

        for item in &textrooms {
            if !locations.is_empty() {
                let index = rng.gen_range(0..locations.len());

                let y_pos = &locations[index].0;
                let x_pos = &locations[index].1;
                map.data[[*y_pos, *x_pos]].room_type = RoomType::Text;
                map.data[[*y_pos, *x_pos]].item_id = Some(item.to_string());

                locations.remove(index);
            } else {
                return Err("map has no valid spot to place a text room".into());
            }
        }

        if area == 0 {
            // place the PChip room
            let index = rng.gen_range(0..locations.len());
            let y_pos = &locations[index].0;
            let x_pos = &locations[index].1;
            map.data[[*y_pos, *x_pos]].block_set = Some(items::item_library::get_p_chip_room());
            map.data[[*y_pos, *x_pos]].chip_tile = true;
        }
        Ok(())
    }

    pub fn place_cardinal_directions(
        &self,
        map: &mut Map,
        rng: &mut ChaCha8Rng,
    ) -> Result<(), TGLError> {
        // Find all rooms on ring
        let err = "an option that was expected to be Some was None in place_carrdinal directions";
        let mut ring_rooms = Vec::new();
        for y_pos in 0..24 {
            for x_pos in 0..24 {
                if map.data[[y_pos, x_pos]].area.ok_or(tgl_error(err))? == 0
                    && map.data[[y_pos, x_pos]].avoid_special
                    && map.data[[y_pos, x_pos]].room_type == RoomType::Normal
                    && map.data[[y_pos, x_pos]].accessible
                {
                    ring_rooms.push((y_pos, x_pos));
                }
            }
        }

        // Find the farthest points
        let (mut north_y, mut south_y, mut west_x, mut east_x) = (None, None, None, None);
        for &(y_pos, x_pos) in &ring_rooms {
            if north_y.is_none() || y_pos < north_y.ok_or(tgl_error(err))? {
                north_y = Some(y_pos);
            }
            if south_y.is_none() || y_pos > south_y.ok_or(tgl_error(err))? {
                south_y = Some(y_pos);
            }
            if west_x.is_none() || x_pos < west_x.ok_or(tgl_error(err))? {
                west_x = Some(x_pos);
            }
            if east_x.is_none() || x_pos > east_x.ok_or(tgl_error(err))? {
                east_x = Some(x_pos);
            }
        }

        // Find the mean position along each side
        let (mut north_sum, mut north_tiles, mut south_sum, mut south_tiles) = (0, 0, 0, 0);
        let (mut west_sum, mut west_tiles, mut east_sum, mut east_tiles) = (0, 0, 0, 0);

        for &(y_pos, x_pos) in &ring_rooms {
            if Some(y_pos) == north_y {
                north_sum += x_pos;
                north_tiles += 1;
            }
            if Some(y_pos) == south_y {
                south_sum += x_pos;
                south_tiles += 1;
            }
            if Some(x_pos) == west_x {
                west_sum += y_pos;
                west_tiles += 1;
            }
            if Some(x_pos) == east_x {
                east_sum += y_pos;
                east_tiles += 1;
            }
        }

        let north_avg = north_sum as f64 / north_tiles as f64;
        let south_avg = south_sum as f64 / south_tiles as f64;
        let west_avg = west_sum as f64 / west_tiles as f64;
        let east_avg = east_sum as f64 / east_tiles as f64;

        // Find the closest tile to each tile
        let (mut north_room, mut north_room_distance) = (None, None);
        let (mut south_room, mut south_room_distance) = (None, None);
        let (mut west_room, mut west_room_distance) = (None, None);
        let (mut east_room, mut east_room_distance) = (None, None);

        for &(y_pos, x_pos) in &ring_rooms {
            if Some(y_pos) == north_y {
                let distance_away = (north_avg - x_pos as f64).abs();
                if (north_room_distance.is_none()
                    || distance_away < north_room_distance.ok_or(tgl_error(err))?)
                    || ((north_avg - x_pos as f64).abs()
                        == north_room_distance.ok_or(tgl_error(err))?
                        && rng.gen_bool(0.5))
                {
                    north_room = Some((y_pos, x_pos));
                    north_room_distance = Some(distance_away);
                }
            }

            if Some(y_pos) == south_y {
                let distance_away = (south_avg - x_pos as f64).abs();
                if (south_room_distance.is_none()
                    || distance_away < south_room_distance.ok_or(tgl_error(err))?)
                    || ((south_avg - x_pos as f64).abs()
                        == south_room_distance.ok_or(tgl_error(err))?
                        && rng.gen_bool(0.5))
                {
                    south_room = Some((y_pos, x_pos));
                    south_room_distance = Some(distance_away);
                }
            }

            if Some(x_pos) == west_x {
                let distance_away = (west_avg - y_pos as f64).abs();
                if (west_room_distance.is_none()
                    || distance_away < west_room_distance.ok_or(tgl_error(err))?)
                    || ((west_avg - y_pos as f64).abs()
                        == west_room_distance.ok_or(tgl_error(err))?
                        && rng.gen_bool(0.5))
                {
                    west_room = Some((y_pos, x_pos));
                    west_room_distance = Some(distance_away);
                }
            }

            if Some(x_pos) == east_x {
                let distance_away = (east_avg - y_pos as f64).abs();
                if (east_room_distance.is_none()
                    || distance_away < east_room_distance.ok_or(tgl_error(err))?)
                    || ((east_avg - y_pos as f64).abs()
                        == east_room_distance.ok_or(tgl_error(err))?
                        && rng.gen_bool(0.5))
                {
                    east_room = Some((y_pos, x_pos));
                    east_room_distance = Some(distance_away);
                }
            }
        }

        if let Some((y, x)) = north_room {
            map.data[[y, x]].block_set = Some(maze::items::item_library::get_cardinal_letter("N")?);
        }
        if let Some((y, x)) = south_room {
            map.data[[y, x]].block_set = Some(maze::items::item_library::get_cardinal_letter("S")?);
        }
        if let Some((y, x)) = east_room {
            map.data[[y, x]].block_set = Some(maze::items::item_library::get_cardinal_letter("E")?);
        }
        if let Some((y, x)) = west_room {
            map.data[[y, x]].block_set = Some(maze::items::item_library::get_cardinal_letter("W")?);
        }
        Ok(())
    }

    fn place_corridor(
        &self,
        corridor_id: i32,
        locations: &mut Vec<(usize, usize)>,
        map: &mut Map,
        rng: &mut ChaCha8Rng,
    ) -> Result<(), TGLError> {
        if !locations.is_empty() {
            let index = rng.gen_range(0..locations.len());

            let y_pos = &locations[index].0;
            let x_pos = &locations[index].1;
            map.data[[*y_pos, *x_pos]].room_type = RoomType::Corridor;
            map.data[[*y_pos, *x_pos]].enemy_type = corridor_id;

            locations.remove(index);
        } else {
            return Err("map has no valid spot to place a corridor".into());
        }
        Ok(())
    }

    fn placestarting_point_rooms(&self, map: &mut Map) -> Result<(), TGLError> {
        let err = "area was None in placestarting_point_rooms";
        for y_pos in 0..24 {
            for x_pos in 0..24 {
                if (map.data[[y_pos, x_pos]].area.ok_or(tgl_error(err))? >= 1
                    && map.data[[y_pos, x_pos]].area.ok_or(tgl_error(err))? <= 10)
                    && (map.data[[y_pos, x_pos]].starting_point)
                {
                    if map.data[[y_pos, x_pos]].area.ok_or(tgl_error(err))? == 1 {
                        map.data[[y_pos, x_pos]].room_type = RoomType::Corridor;
                        map.data[[y_pos, x_pos]].enemy_type = 1;
                    } else {
                        map.data[[y_pos, x_pos]].room_type = RoomType::Save;
                    }
                }
            }
        }
        Ok(())
    }

    fn populate_enemies(&self, map: &mut Map, empty_room_odds: u8, rng: &mut ChaCha8Rng) {
        for y_pos in 0..24 {
            for x_pos in 0..24 {
                if map.data[[y_pos, x_pos]].accessible
                    && (map.data[[y_pos, x_pos]].room_type == RoomType::Normal)
                {
                    if empty_room_odds > 0 {
                        if rng.gen_range(1..=empty_room_odds) != 1 {
                            map.data[[y_pos, x_pos]].enemy_type = rng.gen_range(1..=47);
                        }
                    } else {
                        map.data[[y_pos, x_pos]].enemy_type = rng.gen_range(1..=47);
                    }
                }
            }
        }

        //enemy table
        //01 blue bubble
        ////02 red carpet
        ////03 red carpet and blue bubble
        ////04 yellow ball
        ////05 yellow ball and red and blue hockey pucks
        ////06 red and blue hockey pucks
        ////07 red and blue spiders
        ////08 those tall alien dudes
        ////09 5 yellow bats
        ////0A blue and green balls
        ////0B red and orange balls
        ////0C yellow bats and red hockey pucks
        ////0D single carrot
        ////0E Blue balls and red spiders
        ////0F Transformers and red carpet
        ////10 3 yellow bats
        ////11 multiplication spider
        ////12 1 carrot and 2 tall alien transformer dudes
        ////13 red carpets and 2 blue spinny flowers
        ////14 those tall pointy hermet crab things
        ////15 2 carrots
        ////16 4 bats 3 hermet crabs
        ////17 3 hermet crabs 2 green balls
        ////18 2 red balls
        ////19 1 ice cube a bunch of blue spinny flowers
        ////1A bunch of red small spiders
        ////1B bunch of multiplication ice cubes
        ////1C 2 multiplication ice cubes
        ////1D balls of every color
        ////1E 2 vertical worm things
        ////1F 2 small blue spinny flowers
        ////20 4 yellow bats
        ////21 4 carrots
        ////22 red and blue  hockey pucks, and 2 tall transformer aliens
        ////23 2 big boss spiders
        ////24 a couple spinny flowers, a couple blue hockey pucks
        ////25 bats and spinnys
        ////26 red carpets again
        ////27 bubble dropping robot
        ////28 falling moons
        ////29 bunch of green balls
        ////2A bunch of balls of every color
        ////2B bubble dropping robots except now there's 2 of them
        ////2C vertical worms and red hockey pucks
        ////2D falling moons and red carpets
        ////2E falling moons and one blue spider boss
        ////2F one red spider boss and 2 blue spinnies
    }

    fn place_area_decorations(&self, map: &mut Map, rng: &mut ChaCha8Rng) -> Result<(), TGLError> {
        let mut starting_rooms = vec![];
        for y_pos in 0..24 {
            for x_pos in 0..24 {
                if (map.data[[y_pos, x_pos]].starting_point)
                    && (map.data[[y_pos, x_pos]]
                        .area
                        .ok_or(tgl_error("area was None in place_area_decorations"))?
                        != 0)
                {
                    starting_rooms.push((y_pos, x_pos));
                }
            }
        }

        for &(y_pos, x_pos) in &starting_rooms {
            if map.data[[y_pos, x_pos]].exit_up
                && map.data[[y_pos - 1, x_pos]].room_type == RoomType::Normal
                && map.data[[y_pos - 1, x_pos]].block_set.is_none()
            {
                let chips = rng.gen_range(0..=1); // do I want chips;
                if chips == 0 {
                    // No
                    map.data[[y_pos - 1, x_pos]].block_set = Some(
                        maze::items::item_library::get_random_room_block(false, 1, 2, rng)?,
                    );
                    map.data[[y_pos - 1, x_pos]].chip_tile = false;
                } else {
                    map.data[[y_pos - 1, x_pos]].block_set = Some(
                        maze::items::item_library::get_random_room_block(true, 1, 2, rng)?,
                    );
                    map.data[[y_pos - 1, x_pos]].chip_tile = true;
                }
            }

            if map.data[[y_pos, x_pos]].exit_down
                && map.data[[y_pos + 1, x_pos]].room_type == RoomType::Normal
                && map.data[[y_pos + 1, x_pos]].block_set.is_none()
            {
                let chips = rng.gen_range(0..=1); // Do I want chips;
                if chips == 0 {
                    // No
                    map.data[[y_pos + 1, x_pos]].block_set = Some(
                        maze::items::item_library::get_random_room_block(false, 1, 1, rng)?,
                    );
                    map.data[[y_pos + 1, x_pos]].chip_tile = false;
                } else {
                    map.data[[y_pos + 1, x_pos]].block_set = Some(
                        maze::items::item_library::get_random_room_block(true, 1, 1, rng)?,
                    );
                    map.data[[y_pos + 1, x_pos]].chip_tile = true;
                }
            }

            if map.data[[y_pos, x_pos]].exit_left
                && map.data[[y_pos, x_pos - 1]].room_type == RoomType::Normal
                && map.data[[y_pos, x_pos - 1]].block_set.is_none()
            {
                let chips = rng.gen_range(0..=1); // Do I want chips;
                if chips == 0 {
                    // No
                    map.data[[y_pos, x_pos - 1]].block_set = Some(
                        maze::items::item_library::get_random_room_block(false, 1, 4, rng)?,
                    );
                    map.data[[y_pos, x_pos - 1]].chip_tile = false;
                } else {
                    map.data[[y_pos, x_pos - 1]].block_set = Some(
                        maze::items::item_library::get_random_room_block(true, 1, 4, rng)?,
                    );
                    map.data[[y_pos, x_pos - 1]].chip_tile = true;
                }
            }

            if map.data[[y_pos, x_pos]].exit_right
                && map.data[[y_pos, x_pos + 1]].room_type == RoomType::Normal
                && map.data[[y_pos, x_pos + 1]].block_set.is_none()
            {
                let chips = rng.gen_range(0..=1); // Do I want chips;
                if chips == 0 {
                    // No
                    map.data[[y_pos, x_pos + 1]].block_set = Some(
                        maze::items::item_library::get_random_room_block(false, 1, 3, rng)?,
                    );
                    map.data[[y_pos, x_pos + 1]].chip_tile = false;
                } else {
                    map.data[[y_pos, x_pos + 1]].block_set = Some(
                        maze::items::item_library::get_random_room_block(true, 1, 3, rng)?,
                    );
                    map.data[[y_pos, x_pos + 1]].chip_tile = true;
                }
            }
        }
        Ok(())
    }

    fn place_corridor_decorations(
        &self,
        map: &mut Map,
        rng: &mut ChaCha8Rng,
    ) -> Result<(), TGLError> {
        let mut corridor = vec![];
        for y_pos in 0..24 {
            for x_pos in 0..24 {
                if map.data[[y_pos, x_pos]].room_type == RoomType::Corridor {
                    corridor.push((y_pos, x_pos));
                }
            }
        }

        for &(y_pos, x_pos) in &corridor {
            if map.data[[y_pos, x_pos]].exit_up
                && map.data[[y_pos - 1, x_pos]].room_type == RoomType::Normal
                && map.data[[y_pos - 1, x_pos]].block_set.is_none()
            {
                map.data[[y_pos - 1, x_pos]].block_set = Some(
                    maze::items::item_library::get_random_room_block(false, 2, 2, rng)?,
                );
                map.data[[y_pos - 1, x_pos]].chip_tile = false;
            }

            if map.data[[y_pos, x_pos]].exit_down
                && map.data[[y_pos + 1, x_pos]].room_type == RoomType::Normal
                && map.data[[y_pos + 1, x_pos]].block_set.is_none()
            {
                map.data[[y_pos + 1, x_pos]].block_set = Some(
                    maze::items::item_library::get_random_room_block(false, 2, 1, rng)?,
                );
                map.data[[y_pos + 1, x_pos]].chip_tile = false;
            }

            if map.data[[y_pos, x_pos]].exit_left
                && map.data[[y_pos, x_pos - 1]].room_type == RoomType::Normal
                && map.data[[y_pos, x_pos - 1]].block_set.is_none()
            {
                map.data[[y_pos, x_pos - 1]].block_set = Some(
                    maze::items::item_library::get_random_room_block(false, 2, 4, rng)?,
                );
                map.data[[y_pos, x_pos - 1]].chip_tile = false;
            }

            if map.data[[y_pos, x_pos]].exit_right
                && map.data[[y_pos, x_pos + 1]].room_type == RoomType::Normal
                && map.data[[y_pos, x_pos + 1]].block_set.is_none()
            {
                map.data[[y_pos, x_pos + 1]].block_set = Some(
                    maze::items::item_library::get_random_room_block(false, 2, 3, rng)?,
                );
                map.data[[y_pos, x_pos + 1]].chip_tile = false;
            }
        }
        Ok(())
    }

    fn place_random_decorations(
        &self,
        map: &mut Map,
        decoration_odds: u8,
        chip_odds: u8,
        rng: &mut ChaCha8Rng,
    ) -> Result<(), TGLError> {
        for y_pos in 0..24 {
            for x_pos in 0..24 {
                if (map.data[[y_pos, x_pos]].accessible
                    && (map.data[[y_pos, x_pos]].room_type == RoomType::Normal))
                    && map.data[[y_pos, x_pos]].block_set.is_none()
                {
                    let mut decorate = false;
                    let mut usechips = false;

                    if decoration_odds > 0 {
                        if rng.gen_range(1..=decoration_odds) == 1 {
                            decorate = true;
                        }
                    } else {
                        decorate = true;
                    }

                    if decorate {
                        if chip_odds > 0 {
                            if rng.gen_range(1..=chip_odds) == 1 {
                                usechips = true;
                            }
                        } else {
                            usechips = true;
                        }

                        if usechips {
                            map.data[[y_pos, x_pos]].block_set = Some(
                                maze::items::item_library::get_random_room_block(true, 0, 0, rng)?,
                            );
                            map.data[[y_pos, x_pos]].chip_tile = true;
                        } else {
                            map.data[[y_pos, x_pos]].block_set = Some(
                                maze::items::item_library::get_random_room_block(false, 0, 0, rng)?,
                            );
                            map.data[[y_pos, x_pos]].chip_tile = false;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn create_list_of_suitable_rooms(
        &self,
        map: &Map,
        area: i32,
        discard_special: bool,
        allow_overwrite_entry: bool,
    ) -> Result<Vec<(usize, usize)>, TGLError> {
        let mut suitable_rooms = vec![];

        for y_pos in 0..24 {
            for x_pos in 0..24 {
                if (map.data[[y_pos, x_pos]]
                    .area
                    .ok_or(tgl_error("area was none in creat_list_of_suitable_rooms"))?
                    == area)
                    && (map.data[[y_pos, x_pos]].accessible)
                    && map.data[[y_pos, x_pos]].block_set.is_none()
                    && ((map.data[[y_pos, x_pos]].room_type == RoomType::Normal)
                        || (area != 0
                            && allow_overwrite_entry
                            && map.data[[y_pos, x_pos]].room_type == RoomType::Save
                            && map.data[[y_pos, x_pos]].starting_point))
                    && (!discard_special || !map.data[[y_pos, x_pos]].avoid_special)
                    && (area != 0 || !map.data[[y_pos, x_pos]].starting_point)
                {
                    suitable_rooms.push((y_pos, x_pos));
                }
            }
        }
        Ok(suitable_rooms)
    }

    fn add_connections(
        &self,
        map: &mut Map,
        zone: i32,
        desired_connections: i32,
        one_way: bool,
        portal_only: bool,
        rng: &mut ChaCha8Rng,
    ) -> Result<(), TGLError> {
        let err = "Area was none in add_connections";
        let mut desired_connections = desired_connections;
        // find all the points in the area
        let mut rooms_in_zone = vec![];
        for y_pos in 0..24 {
            for x_pos in 0..24 {
                if (map.data[[y_pos, x_pos]].area.ok_or(tgl_error(err))? == zone)
                    && (map.data[[y_pos, x_pos]].accessible)
                {
                    rooms_in_zone.push((y_pos, x_pos));
                }
            }
        }

        while desired_connections >= 1 && !rooms_in_zone.is_empty() {
            // pick a point at random
            let index = rng.gen_range(0..rooms_in_zone.len());
            let room_to_edit = &rooms_in_zone[index];
            // pick a direction at random
            let desired_direction = rng.gen_range(0..=3);

            let (y_pos, x_pos) = *room_to_edit;

            let mut can_go_up = y_pos > 0
                && map.data[[y_pos - 1, x_pos]].area.ok_or(tgl_error(err))? == zone
                && map.data[[y_pos - 1, x_pos]].accessible
                && !map.data[[y_pos, x_pos]].exit_up;
            let mut can_go_left = x_pos > 0
                && map.data[[y_pos, x_pos - 1]].area.ok_or(tgl_error(err))? == zone
                && map.data[[y_pos, x_pos - 1]].accessible
                && !map.data[[y_pos, x_pos]].exit_left;
            let mut can_go_right = x_pos < 24 - 1
                && map.data[[y_pos, x_pos + 1]].area.ok_or(tgl_error(err))? == zone
                && map.data[[y_pos, x_pos + 1]].accessible
                && !map.data[[y_pos, x_pos]].exit_right;
            let mut can_go_down = y_pos < 24 - 1
                && map.data[[y_pos + 1, x_pos]].area.ok_or(tgl_error(err))? == zone
                && map.data[[y_pos + 1, x_pos]].accessible
                && !map.data[[y_pos, x_pos]].exit_down;

            if portal_only {
                let this_room_type = &map.data[[y_pos, x_pos]].room_type;
                let good_rooms = [
                    RoomType::Save,
                    RoomType::Corridor,
                    RoomType::Text,
                    RoomType::MultiShop,
                    RoomType::SingleShop,
                ]; // i'll do good rooms only because it's possible I could add a room type

                if can_go_up {
                    let other_room_typee = &map.data[[y_pos - 1, x_pos]].room_type;
                    can_go_up = good_rooms.contains(this_room_type)
                        || good_rooms.contains(other_room_typee);
                }
                if can_go_left {
                    let other_room_typee = &map.data[[y_pos, x_pos - 1]].room_type;
                    can_go_left = good_rooms.contains(this_room_type)
                        || good_rooms.contains(other_room_typee);
                }
                if can_go_right {
                    let other_room_typee = &map.data[[y_pos, x_pos + 1]].room_type;
                    can_go_right = good_rooms.contains(this_room_type)
                        || good_rooms.contains(other_room_typee);
                }
                if can_go_down {
                    let other_room_type = &map.data[[y_pos + 1, x_pos]].room_type;
                    can_go_down =
                        good_rooms.contains(this_room_type) || good_rooms.contains(other_room_type);
                }
            }

            if !can_go_up && !can_go_left && !can_go_right && !can_go_down {
                rooms_in_zone.remove(index);
            } else {
                match desired_direction {
                    0 => {
                        if can_go_up {
                            // we can grow here oh boy
                            map.data[[y_pos, x_pos]].exit_up = true;
                            if !one_way {
                                map.data[[y_pos - 1, x_pos]].exit_down = true;
                            }
                            desired_connections -= 1;
                        }
                    }

                    1 => {
                        if can_go_left {
                            // We can grow here oh boy
                            map.data[[y_pos, x_pos]].exit_left = true;
                            if !one_way {
                                map.data[[y_pos, x_pos - 1]].exit_right = true;
                            }
                            desired_connections -= 1;
                        }
                    }

                    2 => {
                        if can_go_right {
                            // We can grow here oh boy
                            map.data[[y_pos, x_pos]].exit_right = true;
                            if !one_way {
                                map.data[[y_pos, x_pos + 1]].exit_left = true;
                            }
                            desired_connections -= 1;
                        }
                    }

                    3 => {
                        if can_go_down {
                            // We can grow here oh boy
                            map.data[[y_pos, x_pos]].exit_down = true;
                            if !one_way {
                                map.data[[y_pos + 1, x_pos]].exit_up = true;
                            }
                            desired_connections -= 1;
                        }
                    }

                    _ => (),
                }
            }
        }
        Ok(())
    }
}
