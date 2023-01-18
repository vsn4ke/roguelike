pub struct RoomBasedSpawner {}

impl super::MetaMapBuilder for RoomBasedSpawner {
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        if let Some(rooms) = &data.rooms {
            for room in rooms.iter().skip(1) {
                super::spawner::spawn_in_room(&data.map, room, &mut data.spawn_list);
            }
        } else {
            panic!("Room Based Spawning only works after rooms have been created");
        }
    }
}

impl RoomBasedSpawner {
    #[allow(dead_code)]
    pub fn new() -> Box<RoomBasedSpawner> {
        Box::new(RoomBasedSpawner {})
    }
}
