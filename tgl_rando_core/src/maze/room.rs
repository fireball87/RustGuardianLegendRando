#[derive(PartialEq, Default, Clone)]
pub enum RoomType {
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
    pub area: Option<i32>, // Number between 0 and 10, or None if undefined
    pub accessible: bool,
    pub starting_point: bool, // Starting point is the connection to area 0
    pub exit_up: bool,
    pub exit_down: bool,
    pub exit_left: bool,
    pub exit_right: bool,
    pub avoid_special: bool, // Special code to avoid adding stuff to the ring
    pub room_type: RoomType, // Enum variant representing room type
    pub block_set: Option<String>, // Shared between item rooms and normal rooms
    pub chip_tile: bool,     // If set, blocks are chip blocks; None for nothing, 0 for blue chips
    pub item_id: Option<String>, // Can also be miniboss ID, reused for shop and text ID
    pub enemy_type: i32,     // 0 for empty, 1-47 for valid enemy values (documented in map)
                             // Also use enemy type for corridor
                             // Contained items, bosses, etc., will be added later
}

impl Room {
    pub(crate) fn count_bytes(&self) -> i32 {
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
                    value += 2;
                }
                if self.enemy_type != 0 {
                    value += 1;
                }
                value
            }
        }
    }
}
