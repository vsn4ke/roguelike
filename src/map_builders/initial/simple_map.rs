use bracket_lib::{random::RandomNumberGenerator, terminal::Rect};

use super::InitialMapBuilder;

pub struct SimpleMapBuilder {}

impl InitialMapBuilder for SimpleMapBuilder {
    #[allow(dead_code)]
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        let mut rng = RandomNumberGenerator::new();
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;
        let mut rooms: Vec<Rect> = Vec::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);

            let x = rng.range(0, data.map.width - w);
            let y = rng.range(0, data.map.height - h);

            let new_room = Rect::with_size(x, y, w, h);
            let mut ok = true;

            for other_room in rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }

            if ok {
                rooms.push(new_room);
            }
        }
        data.rooms = Some(rooms);
    }
}

impl SimpleMapBuilder {
    #[allow(dead_code)]
    pub fn new() -> Box<SimpleMapBuilder> {
        Box::new(SimpleMapBuilder {})
    }
}
