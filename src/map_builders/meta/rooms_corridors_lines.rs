use std::collections::HashSet;

use bracket_lib::{
    prelude::Algorithm2D,
    terminal::{line2d_bresenham, DistanceAlg, Rect},
};

pub struct StraightLineCorridors {}

impl super::MetaMapBuilder for StraightLineCorridors {
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        let rooms: Vec<Rect> = if let Some(r) = &data.rooms {
            r.clone()
        } else {
            panic!("Straight line Corridors require a builder with room structures");
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
                        DistanceAlg::Pythagoras.distance2d(room_center, other_center),
                    ));
                }
            }

            if !room_dist.is_empty() {
                room_dist.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                let dest_center = rooms[room_dist[0].0].center();

                let line = line2d_bresenham(room_center, dest_center);
                let mut corridor = Vec::new();
                for cell in line.iter() {
                    let idx = data.map.point2d_to_index(*cell);
                    if data.map.tiles[idx].surface != super::Surface::Floor {
                        data.map.tiles[idx].surface = super::Surface::Floor;
                        corridor.push(idx);
                    }
                }

                corridors.push(corridor);
                connected.insert(i);
                data.take_snapshot();
            }
        }
        data.corridors = Some(corridors);
    }
}

impl StraightLineCorridors {
    #[allow(dead_code)]
    pub fn new() -> Box<StraightLineCorridors> {
        Box::new(StraightLineCorridors {})
    }
}
