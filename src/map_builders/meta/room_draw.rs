use bracket_lib::{
    prelude::Algorithm2D,
    terminal::{DistanceAlg, Point, Rect},
};

use super::{BuilderMap, RandomGen, Surface};
pub struct RoomDrawer {}

impl super::MetaMapBuilder for RoomDrawer {
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        let mut rng = RandomGen::default();
        let rooms: Vec<Rect> = if let Some(rb) = &data.rooms {
            rb.clone()
        } else {
            panic!("Room drawer require a builder with room structures");
        };
        for room in rooms.iter() {
            match rng.range(0, 4) {
                0 => self.circle(data, room),
                _ => self.rectangle(data, room),
            }

            data.take_snapshot();
        }
    }
}

impl RoomDrawer {
    #[allow(dead_code)]
    pub fn new() -> Box<RoomDrawer> {
        Box::new(RoomDrawer {})
    }

    fn rectangle(&mut self, data: &mut BuilderMap, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = data.map.coord_to_index(x, y);
                if idx > 0 && idx < (data.map.width * data.map.height - 1) as usize {
                    data.map.tiles[idx].surface = Surface::Floor;
                }
            }
        }
    }

    fn circle(&mut self, data: &mut BuilderMap, room: &Rect) {
        let radius = i32::min(room.x2 - room.x1, room.y2 - room.y1) as f32 / 2.0;

        let center = room.center();
        for y in room.y1..=room.y2 {
            for x in room.x1..=room.x2 {
                let p = Point::new(x, y);
                let idx = data.map.point2d_to_index(p);
                let dist = DistanceAlg::Pythagoras.distance2d(center, p);
                if idx > 0
                    && idx < (data.map.width * data.map.height - 1) as usize
                    && dist <= radius
                {
                    data.map.tiles[idx].surface = Surface::Floor;
                }
            }
        }
    }
}
