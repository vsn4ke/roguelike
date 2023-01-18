use bracket_lib::prelude::Algorithm2D;

pub struct RoomBasedStairs {}

impl super::MetaMapBuilder for RoomBasedStairs {
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        if let Some(rooms) = &data.rooms {
            let idx = data.map.point2d_to_index(rooms[rooms.len() - 1].center());
            data.map.tiles[idx].surface = super::Surface::DownStairs;
            data.take_snapshot();
        } else {
            panic!("Room Based Stairs only works after rooms have been created");
        }
    }
}

impl RoomBasedStairs {
    #[allow(dead_code)]
    pub fn new() -> Box<RoomBasedStairs> {
        Box::new(RoomBasedStairs {})
    }
}
