use bracket_lib::{prelude::Algorithm2D, random::RandomNumberGenerator, terminal::Rect};

use super::{
    paint::{paint, Symmetry},
    Surface,
};

pub struct RoomExploder {}

impl super::MetaMapBuilder for RoomExploder {
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        let mut rng = RandomNumberGenerator::new();
        let rooms: Vec<Rect> = if let Some(rb) = &data.rooms {
            rb.clone()
        } else {
            panic!("Room Explosions require a builder with room structures");
        };

        for room in rooms.iter() {
            let start = room.center();
            let diggers = rng.range(-3, 16);
            if diggers <= 0 {
                continue;
            }
            for _ in 0..diggers {
                let mut p = start;
                let mut did_something = false;

                for _ in 0..20 {
                    let idx = data.map.point2d_to_index(p);
                    did_something = data.map.tiles[idx].surface == Surface::Wall;

                    paint(&mut data.map, Symmetry::None, 1, p);
                    data.map.tiles[idx].surface = Surface::DownStairs;

                    match rng.range(1, 5) {
                        1 => {
                            if p.x > 2 {
                                p.x -= 1
                            }
                        }
                        2 => {
                            if p.x < data.map.width - 2 {
                                p.x += 1
                            }
                        }
                        3 => {
                            if p.y > 2 {
                                p.y -= 1
                            }
                        }
                        _ => {
                            if p.y < data.map.height - 2 {
                                p.y += 1
                            }
                        }
                    }
                }

                if did_something {
                    data.take_snapshot();
                }

                for t in data.map.tiles.iter_mut() {
                    if t.surface == Surface::DownStairs {
                        t.surface = Surface::Floor;
                    }
                }
            }
        }
    }
}

impl RoomExploder {
    #[allow(dead_code)]
    pub fn new() -> Box<RoomExploder> {
        Box::new(RoomExploder {})
    }
}
