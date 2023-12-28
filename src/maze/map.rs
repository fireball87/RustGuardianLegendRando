use crate::maze::room::{Room, RoomType};
use ndarray::Array2;

struct Map {
    // The map is an array of Room objects
    // Top left room is (x=0, y=0) in the game
    // It is 24 tiles wide and high, ending on (x=23, y=23)
    data: Array2<Room>, // A 2D array of Room objects
}

impl Map {
    fn new() -> Self {
        let mut data: Array2<Room> = Array2::from_elem((24, 24), Room::default());
        Map { data }
    }

    fn print_areas(&self) {
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

    fn draw_exits(&self) {
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
                                        hex::decode(&item.item_id.clone().unwrap()).unwrap()[0]
                                            - 0x3F
                                    )
                                }
                                RoomType::SingleShop => {
                                    print!(
                                        "s{}",
                                        hex::decode(&item.item_id.clone().unwrap()).unwrap()[0]
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
