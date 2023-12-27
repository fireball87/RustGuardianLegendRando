use crate::maze::room::Room;
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
}
