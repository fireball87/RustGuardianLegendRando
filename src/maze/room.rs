#[derive(PartialEq, Default, Clone)]
enum RoomType {
    #[default]
    Normal,
    Save,
    Corridor,
    Text,
    MultiShop,
    SingleShop,
    Miniboss,
    Item,
}

#[derive(Default, Clone)]
pub struct Room {
    area: Option<i32>, // Number between 0 and 10, or None if undefined
    accessible: bool,
    starting_point: bool, // Starting point is the connection to area 0
    exit_up: bool,
    exit_down: bool,
    exit_left: bool,
    exit_right: bool,
    avoid_special: bool,   // Special code to avoid adding stuff to the ring
    room_type: RoomType,   // Enum variant representing room type
    block_set: Option<()>, // Shared between item rooms and normal rooms
    chip_tile: bool,       // If set, blocks are chip blocks; None for nothing, 0 for blue chips
    item_id: Option<()>,   // Can also be miniboss ID, reused for shop and text ID
    enemy_type: i32,       // 0 for empty, 1-47 for valid enemy values (documented in map)
                           // Also use enemy type for corridor
                           // Contained items, bosses, etc., will be added later
}

impl Room {
    fn count_bytes(&self) -> i32 {
        if !self.accessible {
            return 1;
        }
        match &self.room_type {
            RoomType::Save | RoomType::Corridor => 3,
            RoomType::Text | RoomType::MultiShop | RoomType::SingleShop | RoomType::Miniboss => 4,
            room_type => {
                let mut value = 3;
                if room_type == &RoomType::Item {
                    value += 1;
                }
                if self.block_set.is_some() {
                    // value += 2;
                }
                if self.enemy_type != 0 {
                    value += 1;
                }
                value
            }
        }
    }
}
