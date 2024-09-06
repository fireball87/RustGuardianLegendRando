// 
// pub struct Generator;
// 
// impl Generator {
//     // The Concept map design process
//     // create the map
//     // subdivide the map into areas off a0 (trying to keep a tgl like grid map)
//     // generate a multiple path maze for each area
//     // find all candidates for connections between two touching areas (and select Some based on flags)
//     // number All areas and assign progression with the note that areas that will connect need to be sequential
//     // place rooms and items and stuff, something I can consider more after I do maze gen
// 
//     pub fn run(
//         &self,
//         item_library: Vec<Vec<&'static str>>,
//         small_shop_library: Vec<Vec<&'static str>>,
//         multi_shop_library: Vec<Vec<&'static str>>,
//         secret: bool,
//         min_area_size: usize,
//         max_area_size: usize,
//         desired_connections: usize,
//         desired_one_way_connections: usize,
//         portal_only_one_ways: bool,
//         decoration_odds: u8,
//         chip_odds: u8,
//         empty_room_odds: u8,
//         log: bool,
//     ) -> Result<Map, String> {
//         //create the map
// 
//         let mut map = Map::new();
// 
//         //subdivide the map into areas off a0 (trying to keep a tgl like grid map)
//         self.subdivide_using_template(&mut map);
// 
//         self.shuffle_areas(&mut map);
// 
//         self.growA0ring(&mut map); // i need to place cardinal directions before mapping the starting points, because the starting points will grow out a0 and break the calculation
//         self.placeCardinalDirections(&mut map);
// 
//         self.find_starting_points(&mut map);
// 
//         for i in 1..=10 {
//             self.grow_zone(i, rand(min_area_size, max_area_size), &mut map);
//             self.addConnections(&mut map, i, desired_connections, false, false);
//             self.addConnections(
//                 &mut map,
//                 i,
//                 desired_one_way_connections,
//                 true,
//                 portal_only_one_ways,
//             );
//         }
//         self.grow_zone(0, 50, &mut map);
//         self.addConnections(&mut map, 0, desired_connections, false, false);
//         self.addConnections(
//             &mut map,
//             0,
//             desired_one_way_connections,
//             true,
//             portal_only_one_ways,
//         );
// 
//         self.placeStartingPointRooms(&mut map);
//         self.placeAreaDecorations(&mut map);
// 
//         self.placeStartingTextRoom(&mut map);
// 
//         // place all my items
//         for i in 0..=10 {
//             self.placeImportantRooms(&mut map, small_shop_library, multi_shop_library, i, secret);
//             self.placeItemsAndMinibosses(&mut map, item_library, i, secret);
//             self.placeNonImportantRooms(&mut map, i, secret);
//         }
// 
//         self.placeCorridorDecorations(&mut map);
//         self.placeRandomDecorations(&mut map, decoration_odds, chip_odds);
// 
//         self.populateEnemies(&mut map, empty_room_odds);
//         let bytes = self.countAllRoomBytes(&map);
//         if log {
//             println!("{}", bytes);
//         }
//         if bytes > 1916 {
//             Err("Produced map that is too large".to_string())
//         } else {
//             Ok(map)
//         }
//     }
// 
//     fn countAllRoomBytes(&self, map: &Map) -> usize {
//         let mut total = 0;
//         for y_pos in 0..24 {
//             for x_pos in 0..24 {
//                 total += map.data[y_pos][x_pos].count_bytes();
//             }
//         }
//         total
//     }
// 
//     fn subdivide_using_template(&self, map: &mut Map) {
//         // select from a template
//         let selected_entry = rand(0, DivisionLibrary::entry_count() - 1);
//         let division = DivisionLibrary::get(selected_entry);
// 
//         let shouldFlip = rand(0, 1);
//         if ShouldFlip == 1 {
//             for row in &mut map.data {
//                 row.reverse();
//             }
//         }
// 
//         let RotateTimes = rand(0, 3);
// 
//         match RotateTimes {
//             3 => map.data = self.rotate90(&mut map.data),
//             2 => map.data = self.rotate90(&mut map.data),
//             1 => map.data = self.rotate90(&mut map.data),
// 
//             _ => (),
//         }
//     }
// 
//     fn shuffle_areas(&self, map: &mut Map) {
//         let mut newareas = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//         shuffle(&mut newareas);
// 
//         for row in &mut map.data {
//             for item in row {
//                 if (item.area >= 1) && (item.area <= 10) {
//                     item.area = newareas[item.area as usize - 1];
//                 }
//             }
//         }
//     }
// 
//     fn rotate90(map: &mut [[Room; 24]; 24]) -> Vec<Vec<Room>> {
//         // Implementation of Rotate90 goes here
//         Vec::new()
//     }
// 
//     fn find_starting_points(&self, map: &mut Map) {
//         let mut possible_grow_points = vec![];
// 
//         for y_pos in 0..24 {
//             for x_pos in 0..24 {
//                 if (map.data[y_pos][x_pos].area == -1)
//                 // only borders can be made starting positions
//                 {
//                     let mut above_block = None;
//                     let mut below_block = None;
//                     let mut right_block = None;
//                     let mut left_block = None;
// 
//                     if y_pos > 0 {
//                         above_block = Some(&map.data[y_pos - 1][x_pos]);
//                     }
//                     if y_pos < 23 {
//                         below_block = Some(&map.data[y_pos + 1][x_pos]);
//                     }
//                     if x_pos > 0 {
//                         left_block = Some(&map.data[y_pos][x_pos - 1]);
//                     }
//                     if x_pos < 23 {
//                         right_block = Some(&map.data[y_pos][x_pos + 1]);
//                     }
// 
//                     /*if !(above_block == None || below_block == None || right_block == None || left_block == None)
//                     {
//                         continue;// if nothing we touch is a 0 we're done son, this block isn't a candidate
//                     }*/
//                     if let (Some(above), Some(below)) = (above_block, below_block) {
//                         if above.area == 0 && below.area >= 1 && below.area <= 10 {
//                             possible_grow_points.push((y_pos, x_pos, "up"));
//                             continue;
//                         }
//                     }
// 
//                     if let (Some(below), Some(above)) = (below_block, above_block) {
//                         if below.area == 0 && above.area >= 1 && above.area <= 10 {
//                             possible_grow_points.push((y_pos, x_pos, "down"));
//                             continue;
//                         }
//                     }
// 
//                     if let (Some(left), Some(right)) = (left_block, right_block) {
//                         if left.area == 0 && right.area >= 1 && right.area <= 10 {
//                             possible_grow_points.push((y_pos, x_pos, "left"));
//                             continue;
//                         }
//                     }
// 
//                     if let (Some(right), Some(left)) = (right_block, left_block) {
//                         if right.area == 0 && left.area >= 1 && left.area <= 10 {
//                             possible_grow_points.push((y_pos, x_pos, "right"));
//                             continue;
//                         }
//                     }
//                 }
//             }
//         }
// 
//         // now that we have a list of candidates lets pick one at random for each stage
//         // TODO: this can pick diagonal rooms, make sure you can enter rooms in that setup, if you can i see no reason to fix it
//         for (area, position_array) in possible_grow_points.iter().enumerate() {
//             if position_array.is_empty() {
//                 return Err("Area "
//                     + (area + 1).to_string()
//                     + " did not generate with any possible entry points.");
//             }
// 
//             let index = rand(0, position_array.len());
//             let value = &position_array[index];
// 
//             let y_pos = value.0;
//             let x_pos = value.1;
// 
//             map.data[y_pos][x_pos].area = 0;
//             map.data[y_pos][x_pos].startingpoint = true;
//             map.data[y_pos][x_pos].accessible = true;
// 
//             let dir = value.2;
//             if let Some(dir) = match dir {
//                 "down" => Some(0),
//                 "up" => Some(1),
//                 "right" => Some(2),
//                 "left" => Some(3),
//                 _ => None,
//             } {
//                 match dir {
//                     0 => {
//                         map.data[y_pos][x_pos].exit_up = true;
//                         map.data[y_pos - 1][x_pos].exit_down = true;
//                         map.data[y_pos - 1][x_pos].accessible = true;
//                     }
// 
//                     1 => {
//                         map.data[y_pos][x_pos].exit_down = true;
//                         map.data[y_pos + 1][x_pos].exit_up = true;
//                         map.data[y_pos + 1][x_pos].accessible = true;
//                     }
// 
//                     2 => {
//                         map.data[y_pos][x_pos].exit_right = true;
//                         map.data[y_pos][x_pos + 1].exit_left = true;
//                         map.data[y_pos][x_pos + 1].accessible = true;
//                     }
// 
//                     3 => {
//                         map.data[y_pos][x_pos].exit_left = true;
//                         map.data[y_pos][x_pos - 1].exit_right = true;
//                         map.data[y_pos][x_pos - 1].accessible = true;
//                     }
// 
//                     _ => (),
//                 }
//             } else {
//                 return Err("Error in find starting points");
//             }
//         }
//     }
// 
//     fn grow_zone(&self, zone: usize, desired_size: usize, map: &mut Map) {
//         // find the starting point
// 
//         let mut possible_grow_points = vec![];
//         for y_pos in 0..24 {
//             for x_pos in 0..24 {
//                 if (map.data[y_pos][x_pos].area == zone)
//                     && map.data[y_pos][x_pos].accessible == true
//                 {
//                     possible_grow_points.push((y_pos, x_pos));
//                 }
//             }
//         }
// 
//         while desired_size > 0 && !possible_grow_points.is_empty() {
//             // take a grow point
//             let index = rand(0, possible_grow_points.len());
//             let location = &possible_grow_points[index];
//             let y_pos = location.0;
//             let x_pos = location.1;
// 
//             // pick a direction you wish to grow, 0 up, 1 left, 2 right, 3 down
//             let grow_direction = rand(0, 3);
// 
//             // randomize the preference of the next blocks
//             let sequence = rand(0, 5);
//             match sequence {
//                 0 => {
//                     up = 0;
//                     left = 1;
//                     right = 2;
//                     down = 3;
//                 }
// 
//                 1 => {
//                     up = 0;
//                     left = 1;
//                     down = 2;
//                     right = 3;
//                 }
// 
//                 2 => {
//                     up = 0;
//                     right = 1;
//                     down = 2;
//                     left = 3;
//                 }
// 
//                 3 => {
//                     up = 0;
//                     right = 1;
//                     left = 2;
//                     down = 3;
//                 }
// 
//                 4 => {
//                     up = 0;
//                     down = 1;
//                     left = 2;
//                     right = 3;
//                 }
// 
//                 5 => {
//                     up = 0;
//                     down = 1;
//                     right = 2;
//                     left = 3;
//                 }
// 
//                 _ => (),
//             }
// 
//             let mut grew = false;
//             for _ in 0..4 {
//                 let rem = (grow_direction + _) % 4;
//                 // check to see if i can grow in this direction
//                 match rem {
//                     up => {
//                         if y_pos > 0
//                             && map.data[y_pos - 1][x_pos].area == zone
//                             && !(map.data[y_pos - 1][x_pos].accessible)
//                         {
//                             // we can grow here oh boy
//                             map.data[y_pos][x_pos].exit_up = true;
//                             map.data[y_pos - 1][x_pos].exit_down = true;
//                             map.data[y_pos - 1][x_pos].accessible = true;
//                             possible_grow_points.push((y_pos - 1, x_pos));
//                             grew = true;
//                         }
//                     }
// 
//                     left => {
//                         if x_pos > 0
//                             && map.data[y_pos][x_pos - 1].area == zone
//                             && !(map.data[y_pos][x_pos - 1].accessible)
//                         {
//                             // We can grow here oh boy
//                             map.data[y_pos][x_pos].exit_left = true;
//                             map.data[y_pos][x_pos - 1].exit_right = true;
//                             map.data[y_pos][x_pos - 1].accessible = true;
//                             possible_grow_points.push((y_pos, x_pos - 1));
//                             grew = true;
//                         }
//                     }
// 
//                     right => {
//                         if x_pos < 24
//                             && map.data[y_pos][x_pos + 1].area == zone
//                             && !(map.data[y_pos][x_pos + 1].accessible)
//                         {
//                             // We can grow here oh boy
//                             map.data[y_pos][x_pos].exit_right = true;
//                             map.data[y_pos][x_pos + 1].exit_left = true;
//                             map.data[y_pos][x_pos + 1].accessible = true;
//                             possible_grow_points.push((y_pos, x_pos + 1));
//                             grew = true;
//                         }
//                     }
// 
//                     down => {
//                         if y_pos < 24
//                             && map.data[y_pos + 1][x_pos].area == zone
//                             && !(map.data[y_pos + 1][x_pos].accessible)
//                         {
//                             // We can grow here oh boy
//                             map.data[y_pos][x_pos].exit_down = true;
//                             map.data[y_pos + 1][x_pos].exit_up = true;
//                             map.data[y_pos + 1][x_pos].accessible = true;
//                             possible_grow_points.push((y_pos + 1, x_pos));
// 
//                             grew = true;
//                         }
//                     }
// 
//                     _ => (),
//                 }
//                 if grew {
//                     break;
//                 }
//             }
// 
//             if !grew {
//                 // if I didn't grow, we need to remove this cell from the growable cells list
//                 // possible_grow_points.remove(index);
//             } else {
//                 // Otherwise lower the desired size
//                 desired_size = desired_size - 1;
//             }
// 
//             if !grew {
//                 possible_grow_points.remove(index);
//             } else {
//                 // otherwise lower the desired size
//                 desired_size -= 1;
//             }
//         }
//     }
// 
//     fn growA0ring(&self, map: &mut Map) {
//         // form the outside ring
//         for y_pos in 0..24 {
//             for x_pos in 0..24 {
//                 if (map.data[y_pos][x_pos].area == 0)
//                     && (map.data[y_pos][x_pos].avoid_special)
//                     && (map.data[y_pos][x_pos].room_type == 0)
//                     && (map.data[y_pos][x_pos].accessible)
//                 {
//                     let mut grow_vertically = false;
//                     let mut grow_horizontally = false;
// 
//                     if y_pos > 0 && map.data[y_pos - 1][x_pos].area <= -1
//                         || y_pos < 23 && map.data[y_pos + 1][x_pos].area <= -1
//                     {
//                         grow_horizontally = true;
//                     }
// 
//                     if x_pos > 0 && map.data[y_pos][x_pos - 1].area <= -1
//                         || x_pos < 23 && map.data[y_pos][x_pos + 1].area <= -1
//                     {
//                         grow_vertically = true;
//                     }
// 
//                     if grow_vertically {
//                         map.data[y_pos][x_pos].accessible = true;
//                         map.data[y_pos][x_pos].avoid_special = true;
//                         // grow up if can
//                         if map.data[y_pos - 1][x_pos].area == 0 {
//                             map.data[y_pos][x_pos].exit_up = true;
//                             map.data[y_pos - 1][x_pos].accessible = true;
//                             map.data[y_pos - 1][x_pos].avoid_special = true;
//                             map.data[y_pos - 1][x_pos].exit_down = true;
//                         }
//                         // grow down If can
//                         if map.data[y_pos + 1][x_pos].area == 0 {
//                             map.data[y_pos][x_pos].exit_down = true;
//                             map.data[y_pos + 1][x_pos].accessible = true;
//                             map.data[y_pos + 1][x_pos].avoid_special = true;
//                             map.data[y_pos + 1][x_pos].exit_up = true;
//                         }
//                     }
// 
//                     if grow_horizontally {
//                         map.data[y_pos][x_pos].accessible = true;
//                         map.data[y_pos][x_pos].avoid_special = true;
// 
//                         // grow left if can
//                         if map.data[y_pos][x_pos - 1].area == 0 {
//                             map.data[y_pos][x_pos].exit_left = true;
//                             map.data[y_pos][x_pos - 1].accessible = true;
//                             map.data[y_pos][x_pos - 1].exit_right = true;
//                             map.data[y_pos][x_pos - 1].avoid_special = true;
//                         }
//                         // grow right If can
//                         if map.data[y_pos][x_pos + 1].area == 0 {
//                             map.data[y_pos][x_pos].exit_right = true;
//                             map.data[y_pos][x_pos + 1].accessible = true;
//                             map.data[y_pos][x_pos + 1].exit_left = true;
//                             map.data[y_pos][x_pos + 1].avoid_special = true;
//                         }
//                     }
//                 }
//             }
//         }
//     }
// 
//     fn placeItemsAndMinibosses(
//         &self,
//         map: &mut Map,
//         items_library: &HashMap<usize, Vec<String>>,
//         area: usize,
//         secret: bool,
//     ) {
//         let mut locations = self.createListOfSuitableRooms(map, area, false, false);
// 
//         let mut items_to_place = items_library[&area].clone();
// 
//         // minibosses are the same between secret and the normal items
//         let minibosses_to_place = ItemLibrary::getMiniboss(&area);
// 
//         let mut item_blocksets = ItemLibrary::getItemBlocks().clone();
// 
//         for item in &items_to_place {
//             if !locations.is_empty() {
//                 let index = rand(0, locations.len());
// 
//                 let y_pos = &locations[index].0;
//                 let x_pos = &locations[index].1;
//                 map.data[*y_pos][*x_pos].room_type = 7;
//                 map.data[*y_pos][*x_pos].item_id = item.clone();
//                 map.data[*y_pos][*x_pos].block_set = *item_blocksets
//                     .get(rand(0, item_blocksets.len() - 1))
//                     .unwrap();
//                 locations.remove(index);
//             } else {
//                 return Err("map has no valid spot to place an item");
//             }
//         }
// 
//         for item in &minibosses_to_place {
//             if !locations.is_empty() {
//                 let index = rand(0, locations.len());
// 
//                 let y_pos = &locations[index].0;
//                 let x_pos = &locations[index].1;
//                 map.data[*y_pos][*x_pos].room_type = 6;
//                 map.data[*y_pos][*x_pos].item_id = item.clone();
// 
//                 locations.remove(index);
//             } else {
//                 return Err("map has no valid spot to place a miniboss");
//             }
//         }
//     }
// 
//     fn placeStartingTextRoom(&self, map: &mut Map) {
//         let y_pos = 12;
//         let x_pos = 11;
//         map.data[y_pos][x_pos].room_type = 3;
//         map.data[y_pos][x_pos].item_id = "00".to_string();
//     }
// 
//     fn placeImportantRooms(
//         &self,
//         map: &mut Map,
//         single_shop_library: &HashMap<usize, Vec<Vec<u8>>>,
//         multi_shop_library: &HashMap<usize, Vec<Vec<u8>>>,
//         area: usize,
//         secret: bool,
//     ) {
//         let mut locations = self.createListOfSuitableRooms(map, area, true, true);
// 
//         // place corridors
//         if area == 0 {
//             self.placeCorridor(21, &mut locations, map);
//         } else if area == 1 {
//             self.placeCorridor(11, &mut locations, map);
//         } else if secret && area == 4 && false {
//             // find a sutable c4 location
//             let mut array_copy = locations.clone(); // this actually copies the array
//             shuffle(&mut array_copy);
//             let mut foundRoom = false;
//             for item in &array_copy {
//                 // check the rooms if it is possible
//                 // [4][1]
//                 // [5][2][0]
//                 // 7
// 
//                 // check that the rooms fit on the map
//                 if (item.0 - 1 >= 0) && (item.0 + 1 <= 23) && (item.1 - 2 > 0) {
//                     // if the room can be placed in space check All the rooms
// 
//                     let rooms = vec![
//                         *item,
//                         [item.0 - 1, item.1 - 1],
//                         [item.0, item.1 - 1],
//                         [item.0 + 1, item.1 - 1],
//                         [item.0 - 1, item.1 - 2],
//                         [item.0, item.1 - 2],
//                     ];
//                     let mut working_roomset = true;
// 
//                     for room in rooms {
//                         if map.data[room.0][room.1].area != 4
//                             || !map.data[room.0][room.1].room_type == 0
//                         {
//                             working_roomset = false;
//                             break;
//                         }
//                     }
// 
//                     if working_roomset {
//                         foundRoom = true;
//                         for room in rooms {
//                             map.data[room.0][room.1].accessible = true;
//                             map.data[room.0][room.1].avoid_special = true;
//                         }
// 
//                         // place the text room in map tile marked 0
//                         map.data[item.0][item.1].exit_left = true;
//                         map.data[item.0][item.1].room_type = 3;
//                         map.data[item.0][item.1].item_id = "12".to_string();
// 
//                         // place the corridor in map tile marked 2
//                         map.data[item.0][item.1 - 1].exit_left = true;
//                         map.data[item.0][item.1 - 1].exit_right = true;
//                         map.data[item.0][item.1 - 1].exit_up = true;
//                         map.data[item.0][item.1 - 1].exit_down = true;
//                         map.data[item.0][item.1 - 1].room_type = 2;
//                         map.data[item.0][item.1 - 1].enemy_type = 4;
// 
//                         // exits for 1
//                         map.data[item.0 - 1][item.1 - 1].exit_left = true;
//                         map.data[item.0 - 1][item.1 - 1].exit_down = true;
//                         // exits for 3
//                         map.data[item.0 + 1][item.1 - 1].exit_up = true;
//                         // exits for 4
//                         map.data[item.0 - 1][item.1 - 2].exit_down = true;
//                         map.data[item.0 - 1][item.1 - 2].exit_right = true;
// 
//                         // exits for 5
//                         map.data[item.0][item.1 - 2].exit_up = true;
//                         map.data[item.0][item.1 - 2].exit_right = true;
// 
//                         // remove all rooms from rooms array
// 
//                         for room in &rooms {
//                             if let Some(index) = locations.iter().position(|&r| r == *room) {
//                                 locations.remove(index);
//                             }
//                         }
// 
//                         break;
//                     }
//                 }
//             }
// 
//             if !foundRoom {
//                 return Err("could not place secret C4");
//             }
//         } else {
//             self.placeCorridor(area, &mut locations, map);
//             self.placeCorridor(area + 10, &mut locations, map);
//         }
// 
//         // place single shops
// 
//         let singleshops = single_shop_library[&area].clone();
//         let multishops = multi_shop_library[&area].clone();
// 
//         for item in &singleshops {
//             if !locations.is_empty() {
//                 let index = rand(0, locations.len());
// 
//                 let y_pos = &locations[index].0;
//                 let x_pos = &locations[index].1;
//                 map.data[*y_pos][*x_pos].room_type = 5;
//                 map.data[*y_pos][*x_pos].item_id = item.clone();
// 
//                 locations.remove(index);
//             } else {
//                 return Err("map has no valid spot to place a single_shop");
//             }
//         }
// 
//         // place multishops
//         for item in &multishops {
//             if !locations.is_empty() {
//                 let index = rand(0, locations.len());
// 
//                 let y_pos = &locations[index].0;
//                 let x_pos = &locations[index].1;
//                 map.data[*y_pos][*x_pos].room_type = 4;
//                 map.data[*y_pos][*x_pos].item_id = item.clone();
// 
//                 locations.remove(index);
//             } else {
//                 return Err("map has no valid spot to place a multi_shop");
//             }
//         }
//     }
// 
//     fn placeNonImportantRooms(&self, map: &mut Map, area: usize, secret: bool) {
//         let mut locations = self.createListOfSuitableRooms(map, area, true, true);
// 
//         // place save room
//         if !locations.is_empty() {
//             let index = rand(0, locations.len());
// 
//             let y_pos = &locations[index].0;
//             let x_pos = &locations[index].1;
//             map.data[*y_pos][*x_pos].room_type = 1;
// 
//             locations.remove(index);
//         } else {
//             return Err("map has no valid spot to place a save room");
//         }
// 
//         // place text rooms
//         let mut textrooms = if secret {
//             SecretLibrary::getTextBlock(&area)
//         } else {
//             ItemLibrary::getTextBlock(&area)
//         };
// 
//         for item in &textrooms {
//             if !locations.is_empty() {
//                 let index = rand(0, locations.len());
// 
//                 let y_pos = &locations[index].0;
//                 let x_pos = &locations[index].1;
//                 map.data[*y_pos][*x_pos].room_type = 3;
//                 map.data[*y_pos][*x_pos].item_id = item.clone();
// 
//                 locations.remove(index);
//             } else {
//                 return Err("map has no valid spot to place a text room");
//             }
//         }
// 
//         if area == 0 {
//             // place the PChip room
//             let index = rand(0, locations.len());
//             let y_pos = &locations[index].0;
//             let x_pos = &locations[index].1;
//             map.data[*y_pos][*x_pos].block_set = ItemLibrary::getPChipRoom();
//             map.data[*y_pos][*x_pos].chip_tile = true;
//         }
//     }
// 
//     fn placeCardinalDirections(&self, map: &mut Map) {
//         // find all rooms on ring
//         let mut ring_rooms = vec![];
//         for y_pos in 0..24 {
//             for x_pos in 0..24 {
//                 if (map.data[y_pos][x_pos].area == 0)
//                     && (map.data[y_pos][x_pos].avoid_special)
//                     && (map.data[y_pos][x_pos].room_type == 0)
//                     && (map.data[y_pos][x_pos].accessible)
//                 {
//                     ring_rooms.push((y_pos, x_pos));
//                 }
//             }
//         }
// 
//         // find the farthest points
//         let mut north_y = None;
//         let mut south_y = None;
//         let mut west_x = None;
//         let mut east_x = None;
// 
//         for &(y_pos, x_pos) in &ring_rooms {
//             // find the northern most point
//             if north_y.is_none() || y_pos < *north_y {
//                 north_y = Some(y_pos);
//             }
//             // find the southern most point
//             if south_y.is_none() || y_pos > *south_y {
//                 south_y = Some(y_pos);
//             }
// 
//             // find the western most point
//             if west_x.is_none() || x_pos < *west_x {
//                 west_x = Some(x_pos);
//             }
//             // find the eastern most point
//             if east_x.is_none() || x_pos > *east_x {
//                 east_x = Some(x_pos);
//             }
//         }
// 
//         // find the mean position along each side
//         let mut north_sum = 0;
//         let mut north_tiles = 0;
//         let mut south_sum = 0;
//         let mut south_tiles = 0;
//         let mut west_sum = 0;
//         let mut west_tiles = 0;
//         let mut east_sum = 0;
//         let mut east_tiles = 0;
// 
//         for &(y_pos, x_pos) in &ring_rooms {
//             if y_pos == *north_y {
//                 north_sum += x_pos;
//                 north_tiles += 1;
//             }
//             if y_pos == *south_y {
//                 south_sum += x_pos;
//                 south_tiles += 1;
//             }
//             if x_pos == *west_x {
//                 west_sum += y_pos;
//                 west_tiles += 1;
//             }
//             if x_pos == *east_x {
//                 east_sum += y_pos;
//                 east_tiles += 1;
//             }
//         }
//         let north_avg = north_sum / north_tiles;
//         let south_avg = south_sum / south_tiles;
//         let west_avg = west_sum / west_tiles;
//         let east_avg = east_sum / east_tiles;
// 
//         // find the closest tile to each tile
//         let mut north_room = None;
//         let mut north_room_distance = None;
//         let mut south_room = None;
//         let mut south_room_distance = None;
//         let mut west_room = None;
//         let mut west_room_distance = None;
//         let mut east_room = None;
//         let mut east_room_distance = None;
// 
//         for &(y_pos, x_pos) in &ring_rooms {
//             if y_pos == *north_y {
//                 let distance_away = (north_avg - x_pos).abs();
//                 if north_room_distance.is_none() || distance_away < *north_room_distance {
//                     north_room = Some((y_pos, x_pos));
//                     north_room_distance = Some(distance_away);
//                 } else if (north_avg - x_pos).abs() == *north_room_distance {
//                     if rand(0, 1) == 1 {
//                         north_room = Some((y_pos, x_pos));
//                         north_room_distance = Some(distance_away);
//                     }
//                 }
//             }
// 
//             if y_pos == *south_y {
//                 let distance_away = (south_avg - x_pos).abs();
//                 if south_room_distance.is_none() || distance_away < *south_room_distance {
//                     south_room = Some((y_pos, x_pos));
//                     south_room_distance = Some(distance_away);
//                 } else if (south_avg - x_pos).abs() == *south_room_distance {
//                     if rand(0, 1) == 1 {
//                         south_room = Some((y_pos, x_pos));
//                         south_room_distance = Some(distance_away);
//                     }
//                 }
//             }
//         }
// 
//         if x_pos == *west_x {
//             let distance_away = (west_avg - y_pos).abs();
//             if west_room_distance.is_none() || distance_away < *west_room_distance {
//                 west_room = Some((y_pos, x_pos));
//                 west_room_distance = Some(distance_away);
//             } else if (west_avg - y_pos).abs() == *west_room_distance {
//                 if rand(0, 1) == 1 {
//                     west_room = Some((y_pos, x_pos));
//                     west_room_distance = Some(distance_away);
//                 }
//             }
//         }
// 
//         if x_pos == *east_x {
//             let distance_away = (east_avg - y_pos).abs();
//             if east_room_distance.is_none() || distance_away < *east_room_distance {
//                 east_room = Some((y_pos, x_pos));
//                 east_room_distance = Some(distance_away);
//             } else if (east_avg - y_pos).abs() == *east_room_distance {
//                 if rand(0, 1) == 1 {
//                     east_room = Some((y_pos, x_pos));
//                     east_room_distance = Some(distance_away);
//                 }
//             }
//         }
// 
//         map.data[north_room.unwrap().0][north_room.unwrap().1].block_set =
//             ItemLibrary::getCardinalLetter("N");
//         map.data[south_room.unwrap().0][south_room.unwrap().1].block_set =
//             ItemLibrary::getCardinalLetter("S");
//         map.data[east_room.unwrap().0][east_room.unwrap().1].block_set =
//             ItemLibrary::getCardinalLetter("E");
//         map.data[west_room.unwrap().0][west_room.unwrap().1].block_set =
//             ItemLibrary::getCardinalLetter("W");
//     }
// 
//     fn placeCorridor(
//         &self,
//         corridor_id: usize,
//         locations: &mut Vec<(usize, usize)>,
//         map: &mut Map,
//     ) {
//         if !locations.is_empty() {
//             let index = rand(0, locations.len());
// 
//             let y_pos = &locations[index].0;
//             let x_pos = &locations[index].1;
//             map.data[*y_pos][*x_pos].room_type = 2;
//             map.data[*y_pos][*x_pos].enemy_type = corridor_id as u8;
// 
//             locations.remove(index);
//         } else {
//             return Err("map has no valid spot to place a corridor");
//         }
//     }
// 
//     fn placeStartingPointRooms(&self, map: &mut Map) {
//         for y_pos in 0..24 {
//             for x_pos in 0..24 {
//                 if (map.data[y_pos][x_pos].area >= 1 && map.data[y_pos][x_pos].area <= 10)
//                     && (map.data[y_pos][x_pos].startingpoint)
//                 {
//                     if area == 1 {
//                         map.data[y_pos][x_pos].room_type = 2;
//                         map.data[y_pos][x_pos].enemy_type = 1;
//                     } else {
//                         map.data[y_pos][x_pos].room_type = 1;
//                     }
//                 }
//             }
//         }
//     }
// 
//     fn populateEnemies(&self, map: &mut Map, empty_room_odds: u8) {
//         for y_pos in 0..24 {
//             for x_pos in 0..24 {
//                 if (map.data[y_pos][x_pos].accessible
//                     && ((map.data[y_pos][x_pos].room_type == 0)
//                         || (area != 0
//                             && allow_overwrite_entry
//                             && map.data[y_pos][x_pos].room_type == 1
//                             && map.data[y_pos][x_pos].startingpoint)))
//                 {
//                     if empty_room_odds > 0 {
//                         if rand(1, empty_room_odds) != 1 {
//                             map.data[y_pos][x_pos].enemy_type = rand(1, 47);
//                         }
//                     } else {
//                         map.data[y_pos][x_pos].enemy_type = rand(1, 47);
//                     }
//                 }
//             }
//         }
//     }
// 
//     fn placeAreaDecorations(&self, map: &mut Map) {
//         let mut starting_rooms = vec![];
//         for y_pos in 0..24 {
//             for x_pos in 0..24 {
//                 if (map.data[y_pos][x_pos].startingpoint) && (map.data[y_pos][x_pos].area != 0) {
//                     starting_rooms.push((y_pos, x_pos));
//                 }
//             }
//         }
// 
//         for &(y_pos, x_pos) in &starting_rooms {
//             if map.data[y_pos][x_pos].exit_up {
//                 if map.data[y_pos - 1][x_pos].room_type == 0
//                     && map.data[y_pos - 1][x_pos].block_set.is_none()
//                 {
//                     let chips = rand(0, 1); // do I want chips;
//                     if chips == 0 {
//                         // No
//                         map.data[y_pos - 1][x_pos].block_set =
//                             ItemLibrary::getRandomRoomBlock(false, 1, 2);
//                         map.data[y_pos - 1][x_pos].chip_tile = false;
//                     } else {
//                         map.data[y_pos - 1][x_pos].block_set =
//                             ItemLibrary::getRandomRoomBlock(true, 1, 2);
//                         map.data[y_pos - 1][x_pos].chip_tile = true;
//                     }
//                 }
//             }
// 
//             if map.data[y_pos][x_pos].exit_down {
//                 if map.data[y_pos + 1][x_pos].room_type == 0
//                     && map.data[y_pos + 1][x_pos].block_set.is_none()
//                 {
//                     let chips = rand(0, 1); // Do I want chips;
//                     if chips == 0 {
//                         // No
//                         map.data[y_pos + 1][x_pos].block_set =
//                             ItemLibrary::getRandomRoomBlock(false, 1, 1);
//                         map.data[y_pos + 1][x_pos].chip_tile = false;
//                     } else {
//                         map.data[y_pos + 1][x_pos].block_set =
//                             ItemLibrary::getRandomRoomBlock(true, 1, 1);
//                         map.data[y_pos + 1][x_pos].chip_tile = true;
//                     }
//                 }
//             }
// 
//             if map.data[y_pos][x_pos].exit_left {
//                 if map.data[y_pos][x_pos - 1].room_type == 0
//                     && map.data[y_pos][x_pos - 1].block_set.is_none()
//                 {
//                     let chips = rand(0, 1); // Do I want chips;
//                     if chips == 0 {
//                         // No
//                         map.data[y_pos][x_pos - 1].block_set =
//                             ItemLibrary::getRandomRoomBlock(false, 1, 4);
//                         map.data[y_pos][x_pos - 1].chip_tile = false;
//                     } else {
//                         map.data[y_pos][x_pos - 1].block_set =
//                             ItemLibrary::getRandomRoomBlock(true, 1, 4);
//                         map.data[y_pos][x_pos - 1].chip_tile = true;
//                     }
//                 }
//             }
// 
//             if map.data[y_pos][x_pos].exit_right {
//                 if map.data[y_pos][x_pos + 1].room_type == 0
//                     && map.data[y_pos][x_pos + 1].block_set.is_none()
//                 {
//                     let chips = rand(0, 1); // Do I want chips;
//                     if chips == 0 {
//                         // No
//                         map.data[y_pos][x_pos + 1].block_set =
//                             ItemLibrary::getRandomRoomBlock(false, 1, 3);
//                         map.data[y_pos][x_pos + 1].chip_tile = false;
//                     } else {
//                         map.data[y_pos][x_pos + 1].block_set =
//                             ItemLibrary::getRandomRoomBlock(true, 1, 3);
//                         map.data[y_pos][x_pos + 1].chip_tile = true;
//                     }
//                 }
//             }
//         }
//     }
// 
//     fn placeCorridorDecorations(&self, map: &mut Map) {
//         let mut corridor = vec![];
//         for y_pos in 0..24 {
//             for x_pos in 0..24 {
//                 if (map.data[y_pos][x_pos].room_type == 2) {
//                     corridor.push((y_pos, x_pos));
//                 }
//             }
//         }
// 
//         for &(y_pos, x_pos) in &corridor {
//             if map.data[y_pos][x_pos].exit_up {
//                 if map.data[y_pos - 1][x_pos].room_type == 0
//                     && map.data[y_pos - 1][x_pos].block_set.is_none()
//                 {
//                     map.data[y_pos - 1][x_pos].block_set =
//                         ItemLibrary::getRandomRoomBlock(false, 2, 2);
//                     map.data[y_pos - 1][x_pos].chip_tile = false;
//                 }
//             }
// 
//             if map.data[y_pos][x_pos].exit_down {
//                 if map.data[y_pos + 1][x_pos].room_type == 0
//                     && map.data[y_pos + 1][x_pos].block_set.is_none()
//                 {
//                     map.data[y_pos + 1][x_pos].block_set =
//                         ItemLibrary::getRandomRoomBlock(false, 2, 1);
//                     map.data[y_pos + 1][x_pos].chip_tile = false;
//                 }
//             }
// 
//             if map.data[y_pos][x_pos].exit_left {
//                 if map.data[y_pos][x_pos - 1].room_type == 0
//                     && map.data[y_pos][x_pos - 1].block_set.is_none()
//                 {
//                     map.data[y_pos][x_pos - 1].block_set =
//                         ItemLibrary::getRandomRoomBlock(false, 2, 4);
//                     map.data[y_pos][x_pos - 1].chip_tile = false;
//                 }
//             }
// 
//             if map.data[y_pos][x_pos].exit_right {
//                 if map.data[y_pos][x_pos + 1].room_type == 0
//                     && map.data[y_pos][x_pos + 1].block_set.is_none()
//                 {
//                     map.data[y_pos][x_pos + 1].block_set =
//                         ItemLibrary::getRandomRoomBlock(false, 2, 3);
//                     map.data[y_pos][x_pos + 1].chip_tile = false;
//                 }
//             }
//         }
//     }
// 
//     fn placeRandomDecorations(&self, map: &mut Map, decoration_odds: u8, chip_odds: u8) {
//         for y_pos in 0..24 {
//             for x_pos in 0..24 {
//                 if (map.data[y_pos][x_pos].accessible && (map.data[y_pos][x_pos].room_type == 0))
//                     && map.data[y_pos][x_pos].block_set.is_none()
//                 {
//                     let mut decorate = false;
//                     let mut usechips = false;
// 
//                     if decoration_odds > 0 {
//                         if rand(1, decoration_odds) == 1 {
//                             decorate = true;
//                         }
//                     } else {
//                         decorate = true;
//                     }
// 
//                     if decorate {
//                         if chip_odds > 0 {
//                             if rand(1, chip_odds) == 1 {
//                                 usechips = true;
//                             }
//                         } else {
//                             usechips = true;
//                         }
// 
//                         if usechips {
//                             map.data[y_pos][x_pos].block_set =
//                                 ItemLibrary::getRandomRoomBlock(true, 0, 0);
//                             map.data[y_pos][x_pos].chip_tile = true;
//                         } else {
//                             map.data[y_pos][x_pos].block_set =
//                                 ItemLibrary::getRandomRoomBlock(false, 0, 0);
//                             map.data[y_pos][x_pos].chip_tile = false;
//                         }
//                     }
//                 }
//             }
//         }
//     }
// 
//     fn createListOfSuitableRooms(
//         &self,
//         map: &Map,
//         area: usize,
//         discard_special: bool,
//         allow_overwrite_entry: bool,
//     ) -> Vec<(usize, usize)> {
//         let mut suitable_rooms = vec![];
// 
//         for y_pos in 0..24 {
//             for x_pos in 0..24 {
//                 if (map.data[y_pos][x_pos].area == area)
//                     && (map.data[y_pos][x_pos].accessible)
//                     && map.data[y_pos][x_pos].block_set.is_none()
//                     && ((map.data[y_pos][x_pos].room_type == 0)
//                         || (area != 0
//                             && allow_overwrite_entry
//                             && map.data[y_pos][x_pos].room_type == 1
//                             && map.data[y_pos][x_pos].startingpoint))
//                 {
//                     if !discard_special || !map.data[y_pos][x_pos].avoid_special {
//                         if area != 0 || !map.data[y_pos][x_pos].startingpoint
//                         // in area 0 don't make any of the starting points miniboss rooms
//                         {
//                             suitable_rooms.push((y_pos, x_pos));
//                         }
//                     }
//                 }
//             }
//         }
//         suitable_rooms
//     }
// 
//     fn addConnections(
//         &self,
//         map: &mut Map,
//         zone: usize,
//         desired_connections: usize,
//         one_way: bool,
//         portal_only: bool,
//     ) {
//         // find all the points in the area
//         let mut rooms_in_zone = vec![];
//         for y_pos in 0..24 {
//             for x_pos in 0..24 {
//                 if (map.data[y_pos][x_pos].area == zone)
//                     && (map.data[y_pos][x_pos].accessible == true)
//                 {
//                     rooms_in_zone.push((y_pos, x_pos));
//                 }
//             }
//         }
// 
//         while desired_connections >= 1 && !rooms_in_zone.is_empty() {
//             // pick a point at random
//             let index = rand(0, rooms_in_zone.len());
//             let room_to_edit = &rooms_in_zone[index];
//             // pick a direction at random
//             let desired_direction = rand(0, 3);
// 
//             let (y_pos, x_pos) = *room_to_edit;
// 
//             let mut can_go_up = y_pos > 0
//                 && map.data[y_pos - 1][x_pos].area == zone
//                 && map.data[y_pos - 1][x_pos].accessible
//                 && !map.data[y_pos][x_pos].exit_up;
//             let mut can_go_left = x_pos > 0
//                 && map.data[y_pos][x_pos - 1].area == zone
//                 && map.data[y_pos][x_pos - 1].accessible
//                 && !map.data[y_pos][x_pos].exit_left;
//             let mut can_go_right = x_pos < 24
//                 && map.data[y_pos][x_pos + 1].area == zone
//                 && map.data[y_pos][x_pos + 1].accessible
//                 && !map.data[y_pos][x_pos].exit_right;
//             let mut can_go_down = y_pos < 24
//                 && map.data[y_pos + 1][x_pos].area == zone
//                 && map.data[y_pos + 1][x_pos].accessible
//                 && !map.data[y_pos][x_pos].exit_down;
// 
//             if portal_only {
//                 let thisRoomType = map.data[y_pos][x_pos].room_type;
//                 let mut goodRooms = vec![1, 2, 3, 4, 5]; // i'll do good rooms only because it's possible I could add a room type
// 
//                 if can_go_up {
//                     let otherRoomType = map.data[y_pos - 1][x_pos].room_type;
//                     can_go_up =
//                         goodRooms.contains(&thisRoomType) || goodRooms.contains(&otherRoomType);
//                 }
//                 if can_go_left {
//                     let otherRoomType = map.data[y_pos][x_pos - 1].room_type;
//                     can_go_left =
//                         goodRooms.contains(&thisRoomType) || goodRooms.contains(&otherRoomType);
//                 }
//                 if can_go_right {
//                     let otherRoomType = map.data[y_pos][x_pos + 1].room_type;
//                     can_go_right =
//                         goodRooms.contains(&thisRoomType) || goodRooms.contains(&otherRoomType);
//                 }
//                 if can_go_down {
//                     let otherRoomType = map.data[y_pos + 1][x_pos].room_type;
//                     can_go_down =
//                         goodRooms.contains(&thisRoomType) || goodRooms.contains(&otherRoomType);
//                 }
//             }
// 
//             if !can_go_up && !can_go_left && !can_go_right && !can_go_down {
//                 rooms_in_zone.remove(index);
//             } else {
//                 match desired_direction {
//                     0 => {
//                         if can_go_up {
//                             // we can grow here oh boy
//                             map.data[y_pos][x_pos].exit_up = true;
//                             if !one_way {
//                                 map.data[y_pos - 1][x_pos].exit_down = true;
//                             }
//                             desired_connections -= 1;
//                         }
//                     }
// 
//                     1 => {
//                         if can_go_left {
//                             // We can grow here oh boy
//                             map.data[y_pos][x_pos].exit_left = true;
//                             if !one_way {
//                                 map.data[y_pos][x_pos - 1].exit_right = true;
//                             }
//                             desired_connections -= 1;
//                         }
//                     }
// 
//                     2 => {
//                         if can_go_right {
//                             // We can grow here oh boy
//                             map.data[y_pos][x_pos].exit_right = true;
//                             if !one_way {
//                                 map.data[y_pos][x_pos + 1].exit_left = true;
//                             }
//                             desired_connections -= 1;
//                         }
//                     }
// 
//                     3 => {
//                         if can_go_down {
//                             // We can grow here oh boy
//                             map.data[y_pos][x_pos].exit_down = true;
//                             if !one_way {
//                                 map.data[y_pos + 1][x_pos].exit_up = true;
//                             }
//                             desired_connections -= 1;
//                         }
//                     }
// 
//                     _ => (),
//                 }
//             }
//         }
//     }
// }
