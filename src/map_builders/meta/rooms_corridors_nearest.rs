use std::collections::HashSet;

use bracket_lib::terminal::{DistanceAlg, Rect};

use super::paint::draw_corridor;
pub struct NearestCorridors {}

impl super::MetaMapBuilder for NearestCorridors {
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        let rooms: Vec<Rect> = if let Some(r) = &data.rooms {
            r.clone()
        } else {
            panic!("Nearest Corridors require a builder with room structures");
        };

        let mut corridors: Vec<Vec<usize>> = Vec::new();
        let mut connected: HashSet<usize> = HashSet::new();
        for (i, room) in rooms.iter().enumerate() {
            let mut room_dist: Vec<(usize, f32)> = Vec::new();
            let room_center = room.center();

            for (j, other_room) in rooms.iter().enumerate() {
                if i != j && !connected.contains(&j) {
                    let other_center = other_room.center();
                    room_dist.push((
                        j,
                        DistanceAlg::PythagorasSquared.distance2d(room_center, other_center),
                    ));
                }
            }

            if !room_dist.is_empty() {
                room_dist.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                let dest_center = rooms[room_dist[0].0].center();
                corridors.push(draw_corridor(
                    &mut data.map,
                    room_center.x,
                    room_center.y,
                    dest_center.x,
                    dest_center.y,
                ));

                connected.insert(i);
                data.take_snapshot();
            }
        }
        data.corridors = Some(corridors);
    }
}

impl NearestCorridors {
    #[allow(dead_code)]
    pub fn new() -> Box<NearestCorridors> {
        Box::new(NearestCorridors {})
    }
}
