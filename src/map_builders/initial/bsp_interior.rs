use super::{super::meta::paint::draw_corridor, BuilderMap, InitialMapBuilder, RandomGen, Surface};
use bracket_lib::terminal::Rect;

const MIN_ROOM_SIZE: i32 = 8;

pub struct BspInteriorBuilder {
    rects: Vec<Rect>,
}

impl InitialMapBuilder for BspInteriorBuilder {
    #[allow(dead_code)]
    fn build_map(&mut self, data: &mut BuilderMap) {
        let mut rng = RandomGen::default();
        let mut rooms = Vec::<Rect>::new();
        self.rects.clear();
        self.rects.push(Rect::with_size(
            1,
            1,
            data.map.width - 2,
            data.map.height - 2,
        ));
        let first_room = self.rects[0];
        self.add_subrects(first_room);

        let rooms_copy = self.rects.clone();
        for (i, r) in rooms_copy.iter().enumerate() {
            let room = *r;

            rooms.push(room);
            for y in room.y1..room.y2 {
                for x in room.x1..room.x2 {
                    let idx = data.map.coord_to_index(x, y);
                    if idx > 0 && idx < ((data.map.width * data.map.height) - 1) as usize {
                        data.map.tiles[idx].surface = Surface::Floor;
                    }
                }
            }
            if i % 4 == 0 {
                data.take_snapshot();
            }
        }

        for i in 0..rooms.len() - 1 {
            let room = rooms[i];
            let next_room = rooms[i + 1];
            let start_x = room.x1 + rng.range(0, i32::abs(room.x1 - room.x2));
            let start_y = room.y1 + rng.range(0, i32::abs(room.y1 - room.y2));
            let end_x = next_room.x1 + rng.range(0, i32::abs(next_room.x1 - next_room.x2));
            let end_y = next_room.y1 + rng.range(0, i32::abs(next_room.y1 - next_room.y2));
            draw_corridor(&mut data.map, start_x, start_y, end_x, end_y);
            if i % 4 == 0 {
                data.take_snapshot();
            }
        }

        data.rooms = Some(rooms);
    }
}

impl BspInteriorBuilder {
    #[allow(dead_code)]
    pub fn new() -> Box<BspInteriorBuilder> {
        Box::new(BspInteriorBuilder { rects: Vec::new() })
    }

    fn add_subrects(&mut self, rect: Rect) {
        let mut rng = RandomGen::default();
        if !self.rects.is_empty() {
            self.rects.remove(self.rects.len() - 1);
        }

        let width = rect.x2 - rect.x1;
        let height = rect.y2 - rect.y1;
        let half_width = width / 2;
        let half_height = height / 2;

        if rng.range(0, 2) == 0 {
            let h1 = Rect::with_size(rect.x1, rect.y1, half_width - 1, height);
            self.rects.push(h1);
            if half_width > MIN_ROOM_SIZE {
                self.add_subrects(h1);
            }
            let h2 = Rect::with_size(rect.x1 + half_width, rect.y1, half_width, height);
            self.rects.push(h2);
            if half_width > MIN_ROOM_SIZE {
                self.add_subrects(h2);
            }
        } else {
            let v1 = Rect::with_size(rect.x1, rect.y1, width, half_height - 1);
            self.rects.push(v1);
            if half_height > MIN_ROOM_SIZE {
                self.add_subrects(v1);
            }
            let v2 = Rect::with_size(rect.x1, rect.y1 + half_height, width, half_height);
            self.rects.push(v2);
            if half_height > MIN_ROOM_SIZE {
                self.add_subrects(v2);
            }
        }
    }
}
