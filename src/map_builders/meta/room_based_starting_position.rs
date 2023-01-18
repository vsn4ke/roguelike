pub struct RoomBasedStartingPosition {}

impl super::MetaMapBuilder for RoomBasedStartingPosition {
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        if let Some(rooms) = &data.rooms {
            data.starting_point = Some(rooms[0].center());
        } else {
            panic!("Room Based Staring Position only works after rooms have been created");
        }
    }
}

impl RoomBasedStartingPosition {
    #[allow(dead_code)]
    pub fn new() -> Box<RoomBasedStartingPosition> {
        Box::new(RoomBasedStartingPosition {})
    }
}
