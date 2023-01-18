use bracket_lib::{random::RandomNumberGenerator, terminal::Rect};

use super::paint::draw_corridor;
pub struct BSPCorridors {}

impl super::MetaMapBuilder for BSPCorridors {
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        let rooms: Vec<Rect> = if let Some(r) = &data.rooms {
            r.clone()
        } else {
            panic!("BSP Corridors require a builder with room structures");
        };
        let mut rng = RandomNumberGenerator::new();

        let mut corridors: Vec<Vec<usize>> = Vec::new();
        for i in 0..rooms.len() - 1 {
            let room = rooms[i];
            let next_room = rooms[i + 1];
            let start_x = room.x1 + rng.range(0, i32::abs(room.x1 - room.x2) - 1);
            let start_y = room.y1 + rng.range(0, i32::abs(room.y1 - room.y2) - 1);
            let end_x = next_room.x1 + rng.range(0, i32::abs(next_room.x1 - next_room.x2) - 1);
            let end_y = next_room.y1 + rng.range(0, i32::abs(next_room.y1 - next_room.y2) - 1);
            corridors.push(draw_corridor(&mut data.map, start_x, start_y, end_x, end_y));
            data.take_snapshot();
        }

        data.corridors = Some(corridors);
    }
}

impl BSPCorridors {
    #[allow(dead_code)]
    pub fn new() -> Box<BSPCorridors> {
        Box::new(BSPCorridors {})
    }
}
