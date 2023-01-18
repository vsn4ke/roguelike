pub struct CorridorSpawner {}

impl CorridorSpawner {
    #[allow(dead_code)]
    pub fn new() -> Box<CorridorSpawner> {
        Box::new(CorridorSpawner {})
    }
}

impl super::MetaMapBuilder for CorridorSpawner {
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        if let Some(corridors) = &data.corridors {
            for c in corridors.iter() {
                super::spawner::spawn_in_region(c, data.map.depth, &mut data.spawn_list);
            }
        } else {
            panic!("Corridor Based Spawning only works after corridors have been created");
        }
    }
}
