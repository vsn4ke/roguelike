use bracket_lib::terminal::Rect;

use super::Surface;

pub struct RoomCornerRounder {}

impl super::MetaMapBuilder for RoomCornerRounder {
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        let rooms: Vec<Rect> = if let Some(r) = &data.rooms {
            r.clone()
        } else {
            panic!("Room Rounding require a builder with room structures");
        };

        for room in rooms.iter() {
            self.fill_if_corner(room.x1 + 1, room.y1 + 1, data);
            self.fill_if_corner(room.x2, room.y1 + 1, data);
            self.fill_if_corner(room.x1 + 1, room.y2, data);
            self.fill_if_corner(room.x2, room.y2, data);

            data.take_snapshot();
        }
    }
}

impl RoomCornerRounder {
    #[allow(dead_code)]
    pub fn new() -> Box<RoomCornerRounder> {
        Box::new(RoomCornerRounder {})
    }

    fn fill_if_corner(&mut self, x: i32, y: i32, data: &mut super::BuilderMap) {
        let w = data.map.width;
        let h = data.map.height;
        let idx = data.map.coord_to_index(x, y);
        let mut walls = 0;

        if x > 0 && data.map.tiles[idx - 1].surface == Surface::Wall {
            walls += 1;
        }
        if y > 0 && data.map.tiles[idx - w as usize].surface == Surface::Wall {
            walls += 1;
        }
        if x < w - 2 && data.map.tiles[idx + 1].surface == Surface::Wall {
            walls += 1;
        }
        if y < h - 2 && data.map.tiles[idx + w as usize].surface == Surface::Wall {
            walls += 1;
        }

        if walls == 2 {
            data.map.tiles[idx].surface = Surface::Wall;
        }
    }
}
