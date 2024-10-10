use crate::maze::room::{Room, RoomType};
use ndarray::Array2;

pub struct Map {
    // The map is an array of Room objects
    // Top left room is (x=0, y=0) in the game
    // It is 24 tiles wide and high, ending on (x=23, y=23)
    pub data: Array2<Room>, // A 2D array of Room objects
}
//
impl Map {
    pub fn new() -> Self {
        let data: Array2<Room> = Array2::from_elem((24, 24), Room::default());
        Map { data }
    }

    pub fn write_hex(&self, log: bool) -> String {
        let mut final_hex = String::new();
        for (_y_pos, row) in self.data.outer_iter().enumerate() {
            for (_x_pos, item) in row.iter().enumerate() {
                if !item.accessible {
                    final_hex += "80";
                    //break 1;
                } else {
                    //1 is down
                    //2 is right
                    //4 is left
                    //8 is up

                    //these are flags so add together, examples picked from game below

                    let mut direction_bit = 0;
                    if item.exit_down {
                        direction_bit += 1;
                    }
                    if item.exit_right {
                        direction_bit += 2;
                    }
                    if item.exit_left {
                        direction_bit += 4;
                    }
                    if item.exit_up {
                        direction_bit += 8;
                    }

                    let area = item.area.unwrap();

                    match item.room_type {
                        //0 normal, 1 save, 2 corridor, 3 text, 4 multi_shop, 5 single_shop, 6 miniboss, 7 item drop
                        RoomType::Normal => {
                            final_hex += &self.normal_room_hex(
                                direction_bit,
                                area,
                                item.enemy_type,
                                item.block_set.clone(),
                                item.chip_tile,
                            )
                        }
                        RoomType::Save => final_hex += &self.save_room_hex(direction_bit, area),
                        RoomType::Corridor => {
                            final_hex += &self.corridor_hex(direction_bit, area, item.enemy_type)
                        }
                        RoomType::Text => {
                            final_hex +=
                                &self.text_hex(direction_bit, area, item.item_id.clone().unwrap())
                        }
                        RoomType::MultiShop => {
                            final_hex += &self.shop_hex(
                                direction_bit,
                                area,
                                item.item_id.clone().unwrap(),
                                true,
                            )
                        }
                        RoomType::SingleShop => {
                            final_hex += &self.shop_hex(
                                direction_bit,
                                area,
                                item.item_id.clone().unwrap(),
                                false,
                            )
                        }
                        RoomType::Miniboss => {
                            final_hex += &self.mini_boss_hex(
                                direction_bit,
                                area,
                                item.item_id.clone().unwrap(),
                            )
                        }
                        RoomType::Item => {
                            final_hex += &self.item_room_hex(
                                direction_bit,
                                area,
                                item.enemy_type,
                                item.block_set.clone().unwrap(),
                                item.item_id.clone().unwrap(),
                            )
                        }
                    }
                }
            }
        }
        if log {
            println!("{}", final_hex.to_uppercase());
        }
        final_hex
    }

    fn item_room_hex(
        &self,
        direction_bit: i32,
        area: i32,
        enemy_type: i32,
        block_type: String,
        item_id: String,
    ) -> String {
        //give me a normal empty room
        let mut required_key =
            self.get_key_from_area_for_rooms_that_could_have_enemies_but_dont(area);
        let meaningless_byte = 0;

        let mut length = 5;
        let mut enemy_string = String::new();
        if enemy_type != 0 {
            required_key = self.get_key_from_area_for_most_rooms(area);

            length += 1;
            enemy_string = format!("{:02X}", enemy_type);
        }

        format!(
            "3{:X}{:X}{:X}{:X}{:X}{}{}{}",
            length,
            direction_bit,
            required_key,
            meaningless_byte,
            area,
            item_id,
            enemy_string,
            block_type
        )
    }

    fn normal_room_hex(
        &self,
        direction_bit: i32,
        area: i32,
        enemy_type: i32,
        block_type: Option<String>,
        chip_tile: bool,
    ) -> String {
        //give me a normal empty room
        let mut required_key =
            self.get_key_from_area_for_rooms_that_could_have_enemies_but_dont(area);

        let mut room_type = 0;

        let mut length = 2;
        let mut enemy_string = String::new();
        if enemy_type != 0 {
            required_key = self.get_key_from_area_for_most_rooms(area);

            length += 1;
            enemy_string = format!("{:02X}", enemy_type);
        }

        let mut block_string = String::new();
        if let Some(block_type) = block_type {
            length += 2;
            block_string = block_type;
            if chip_tile {
                room_type = 7;
                //there's no byte that actually makes sense to control this, my best bet is the 4th byte, as that changes, but 95% of the places just use the key
                //and it's not terribly consistant what the results are otherwise
                //75 7 D 0 8 20 9D94 - area 3
                //75 9 A 0 3 13 9D94 - area 8
                //
                //D 20
                //A 13
            } else {
                room_type = 1;
            }
        }

        let value = format!(
            "{:X}{:X}{:X}{:X}0{:X}{}{}",
            room_type, length, direction_bit, required_key, area, enemy_string, block_string
        );
        //println!("{} {} {} {} {} {} {} {}", room_type, length, direction_bit, required_key, meaningless_byte, area, enemy_string, block_string);
        value
    }

    fn mini_boss_hex(&self, direction_bit: i32, area: i32, item_id: String) -> String {
        let required_key = self.get_key_from_area_for_most_rooms(area);

        format!(
            "43{:X}{:X}1{:X}{}",
            direction_bit, required_key, area, item_id
        )
    }

    fn save_room_hex(&self, direction_bit: i32, area: i32) -> String {
        let required_key = self.get_key_from_area_for_most_rooms(area);

        format!("82{:X}{:X}01", direction_bit, required_key)
    }

    fn corridor_hex(&self, direction_bit: i32, area: i32, corridor: i32) -> String {
        let mut required_key = self.get_key_from_area_for_most_rooms(area);
        if corridor == 1 {
            required_key = 0;
        }

        let corridor_id = format!("{:02X}", 128 + corridor);
        format!("82{:X}{:X}{}", direction_bit, required_key, corridor_id)
    }

    fn shop_hex(
        &self,
        direction_bit: i32,
        area: i32,
        shop_id: String,
        is_multi_shop: bool,
    ) -> String {
        let required_key = self.get_key_from_area_for_rooms_that_could_have_enemies_but_dont(area);
        let meaningless_byte = 0;

        if is_multi_shop {
            format!(
                "A3{:X}{:X}{:X}2{}",
                direction_bit, required_key, meaningless_byte, shop_id
            )
        } else {
            format!(
                "A3{:X}{:X}{:X}6{}",
                direction_bit, required_key, meaningless_byte, shop_id
            )
        }
    }

    fn text_hex(&self, direction_bit: i32, area: i32, text_id: String) -> String {
        let required_key = self.get_key_from_area_for_rooms_that_could_have_enemies_but_dont(area);
        let meaningless_byte = 0;
        format!(
            "A3{:X}{:X}{:X}3{}",
            direction_bit, required_key, meaningless_byte, text_id
        )
    }

    fn get_key_from_area_for_most_rooms(&self, area: i32) -> i32 {
        self.get_key_from_area_for_rooms_that_could_have_enemies_but_dont(area) + 8
    }

    fn get_key_from_area_for_rooms_that_could_have_enemies_but_dont(&self, area: i32) -> i32 {
        match area {
            0 => 0,
            1 | 2 => 1,
            3 => 2,
            4 => 3,
            5 | 6 => 4,
            7 | 8 => 5,
            9 => 6,
            10 => 7,
            _ => {
                panic!("invalid area");
            }
        }
    }

    fn _print_areas(&self) {
        for row in self.data.rows() {
            for item in row.iter() {
                match item.area {
                    None => {
                        print!(" X,");
                    }
                    Some(this_area) => {
                        if this_area < 0 || this_area == 10 {
                            print!("{},", this_area);
                        } else {
                            print!(" {},", this_area);
                        }
                    }
                }
            }
            println!();
        }
    }

    pub fn draw_exits(&self) {
        for row in self.data.rows() {
            for i in 1..=3 {
                for item in row.iter() {
                    if i == 1 {
                        print!("╔═");
                        if item.exit_up {
                            print!("░░");
                        } else {
                            print!("══");
                        }
                        print!("═╗");
                    }
                    if i == 2 {
                        if item.exit_left {
                            print!("░░");
                        } else {
                            print!("║║");
                        }
                        if item.accessible {
                            match item.room_type {
                                //0 normal, 1 save, 2 corridor, 3 text, 4 multi_shop, 5 single_shop, 6 miniboss, 7 item drop
                                RoomType::Save => print!("SV"),
                                RoomType::Corridor => {
                                    let corridor = item.enemy_type;
                                    if corridor < 10 {
                                        print!("X{}", corridor);
                                    } else if corridor == 10 {
                                        print!("XA");
                                    } else if corridor == 21 {
                                        print!("XF");
                                    } else if corridor == 20 {
                                        print!("xA");
                                    } else {
                                        print!("x{}", corridor - 10);
                                    }
                                }
                                RoomType::Text => print!("TX"),
                                RoomType::MultiShop => {
                                    print!(
                                        "S{}",
                                        hex::decode(item.item_id.clone().unwrap()).unwrap()[0]
                                            - 0x3F
                                    )
                                }
                                RoomType::SingleShop => {
                                    print!(
                                        "s{}",
                                        hex::decode(item.item_id.clone().unwrap()).unwrap()[0]
                                            - 0x3A
                                    )
                                }
                                RoomType::Miniboss => print!("{}", &item.item_id.clone().unwrap()),
                                RoomType::Item => print!("{}", &item.item_id.clone().unwrap()),
                                RoomType::Normal => print!("░░"),
                            }
                        } else {
                            print!("╬╬");
                        }
                        if item.exit_right {
                            print!("░░");
                        } else {
                            print!("║║");
                        }
                    }
                    if i == 3 {
                        print!("╚═");
                        if item.exit_down {
                            print!("░░");
                        } else {
                            print!("══");
                        }
                        print!("═╝");
                    }
                }
                println!();
            }
        }
    }
}
