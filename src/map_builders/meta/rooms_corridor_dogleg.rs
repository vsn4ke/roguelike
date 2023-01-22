use bracket_lib::terminal::Rect;

use super::{
    paint::{apply_horizontal_tunnel, apply_vertical_tunnel},
    RandomGen,
};
pub struct DoglegCorridors {}

impl super::MetaMapBuilder for DoglegCorridors {
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        let rooms: Vec<Rect> = if let Some(r) = &data.rooms {
            r.clone()
        } else {
            panic!("Dogleg corridors require a builder with room structures");
        };

        let mut rng = RandomGen::default();

        let mut corridors: Vec<Vec<usize>> = Vec::new();
        for (i, room) in rooms.iter().enumerate() {
            if i == 0 {
                continue;
            }
            let new = room.center();
            let prev = rooms[i - 1].center();
            if rng.range(0, 2) == 1 {
                corridors.push(apply_horizontal_tunnel(
                    &mut data.map,
                    prev.x,
                    new.x,
                    prev.y,
                ));
                corridors.push(apply_vertical_tunnel(&mut data.map, prev.y, new.y, new.x));
            } else {
                corridors.push(apply_vertical_tunnel(&mut data.map, prev.y, new.y, prev.x));
                corridors.push(apply_horizontal_tunnel(&mut data.map, prev.x, new.x, new.y));
            }
            data.take_snapshot();
        }
        data.corridors = Some(corridors);
    }
}

impl DoglegCorridors {
    #[allow(dead_code)]
    pub fn new() -> Box<DoglegCorridors> {
        Box::new(DoglegCorridors {})
    }
}
