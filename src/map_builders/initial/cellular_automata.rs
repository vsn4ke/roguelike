use bracket_lib::random::RandomNumberGenerator;

use super::{BuilderMap, Map, Surface};

pub struct CellularAutomataBuilder {}

impl super::InitialMapBuilder for CellularAutomataBuilder {
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        let mut rng = RandomNumberGenerator::new();
        for y in 1..data.map.height - 1 {
            for x in 1..data.map.width - 1 {
                let roll = rng.range(0, 100);
                let idx = data.map.coord_to_index(x, y);
                if roll > 45 {
                    data.map.tiles[idx].surface = Surface::Floor
                } else {
                    data.map.tiles[idx].surface = Surface::Wall
                }
            }
        }
        data.take_snapshot();

        for _ in 0..15 {
            apply_iteration(&mut data.map);
            data.take_snapshot();
        }
    }
}

impl super::MetaMapBuilder for CellularAutomataBuilder {
    fn build_map(&mut self, data: &mut BuilderMap) {
        apply_iteration(&mut data.map);
        data.take_snapshot();
    }
}

impl CellularAutomataBuilder {
    #[allow(dead_code)]
    pub fn new() -> Box<CellularAutomataBuilder> {
        Box::new(CellularAutomataBuilder {})
    }
}

fn apply_iteration(map: &mut Map) {
    let mut newtile = map.tiles.clone();

    for y in 1..map.height - 1 {
        for x in 1..map.width - 1 {
            let idx = map.coord_to_index(x, y);
            let mut neighbors = 0;
            if map.tiles[idx - 1].surface == Surface::Wall {
                neighbors += 1;
            }
            if map.tiles[idx + 1].surface == Surface::Wall {
                neighbors += 1;
            }
            if map.tiles[idx - map.width as usize].surface == Surface::Wall {
                neighbors += 1;
            }
            if map.tiles[idx + map.width as usize].surface == Surface::Wall {
                neighbors += 1;
            }
            if map.tiles[idx - (map.width as usize - 1)].surface == Surface::Wall {
                neighbors += 1;
            }
            if map.tiles[idx - (map.width as usize + 1)].surface == Surface::Wall {
                neighbors += 1;
            }
            if map.tiles[idx + (map.width as usize - 1)].surface == Surface::Wall {
                neighbors += 1;
            }
            if map.tiles[idx + (map.width as usize + 1)].surface == Surface::Wall {
                neighbors += 1;
            }

            if neighbors > 4 || neighbors == 0 {
                newtile[idx].surface = Surface::Wall;
            } else {
                newtile[idx].surface = Surface::Floor;
            }
        }
    }

    map.tiles = newtile.clone();
}
