use bracket_lib::{prelude::Algorithm2D, terminal::Point};

use super::{
    super::meta::paint::{paint, Symmetry},
    RandomGen, Surface,
};

#[derive(PartialEq, Copy, Clone)]
pub enum DrunkSpawnMode {
    StartingPoint,
    Random,
}

pub struct DrunkardSettings {
    pub spawn_mode: DrunkSpawnMode,
    pub drunken_lifetime: i32,
    pub floor_percent: f32,
    pub brush_size: i32,
    pub symmetry: Symmetry,
}

pub struct DrunkardsWalkBuilder {
    settings: DrunkardSettings,
}

impl DrunkardsWalkBuilder {
    #[allow(dead_code)]
    pub fn new(settings: DrunkardSettings) -> DrunkardsWalkBuilder {
        DrunkardsWalkBuilder { settings }
    }

    #[allow(dead_code)]
    pub fn open_area() -> Box<DrunkardsWalkBuilder> {
        Box::new(DrunkardsWalkBuilder {
            settings: DrunkardSettings {
                spawn_mode: DrunkSpawnMode::StartingPoint,
                drunken_lifetime: 400,
                floor_percent: 0.5,
                brush_size: 1,
                symmetry: Symmetry::None,
            },
        })
    }

    #[allow(dead_code)]
    pub fn open_halls() -> Box<DrunkardsWalkBuilder> {
        Box::new(DrunkardsWalkBuilder {
            settings: DrunkardSettings {
                spawn_mode: DrunkSpawnMode::Random,
                drunken_lifetime: 400,
                floor_percent: 0.5,
                brush_size: 1,
                symmetry: Symmetry::None,
            },
        })
    }

    #[allow(dead_code)]
    pub fn winding_passages() -> Box<DrunkardsWalkBuilder> {
        Box::new(DrunkardsWalkBuilder {
            settings: DrunkardSettings {
                spawn_mode: DrunkSpawnMode::Random,
                drunken_lifetime: 100,
                floor_percent: 0.4,
                brush_size: 1,
                symmetry: Symmetry::None,
            },
        })
    }

    #[allow(dead_code)]
    pub fn fat_passages() -> Box<DrunkardsWalkBuilder> {
        Box::new(DrunkardsWalkBuilder {
            settings: DrunkardSettings {
                spawn_mode: DrunkSpawnMode::Random,
                drunken_lifetime: 100,
                floor_percent: 0.4,
                brush_size: 2,
                symmetry: Symmetry::None,
            },
        })
    }

    #[allow(dead_code)]
    pub fn fearful_symmetry() -> Box<DrunkardsWalkBuilder> {
        Box::new(DrunkardsWalkBuilder {
            settings: DrunkardSettings {
                spawn_mode: DrunkSpawnMode::Random,
                drunken_lifetime: 100,
                floor_percent: 0.4,
                brush_size: 1,
                symmetry: Symmetry::Both,
            },
        })
    }

    fn build(&mut self, data: &mut super::BuilderMap) {
        let mut rng = RandomGen::default();
        let starting_point = Point::new(data.map.width / 2, data.map.height / 2);
        let start_idx = data.map.point2d_to_index(starting_point);
        data.map.tiles[start_idx].surface = Surface::Floor;

        let total_tiles = data.map.width * data.map.height;
        let desired_floor_tiles = (self.settings.floor_percent * total_tiles as f32) as usize;
        let mut floor_tile_count = data
            .map
            .tiles
            .iter()
            .filter(|a| a.surface == Surface::Floor)
            .count();
        let mut digger_count = 0;
        while floor_tile_count < desired_floor_tiles {
            let mut did_something = false;
            let mut drunk = match self.settings.spawn_mode {
                DrunkSpawnMode::StartingPoint => starting_point,
                DrunkSpawnMode::Random => {
                    if digger_count == 0 {
                        starting_point
                    } else {
                        Point::new(
                            rng.range(2, data.map.width - 1),
                            rng.range(2, data.map.height - 1),
                        )
                    }
                }
            };

            for _ in 0..self.settings.drunken_lifetime {
                let drunk_idx = data.map.point2d_to_index(drunk);
                did_something = data.map.tiles[drunk_idx].surface == Surface::Wall;

                paint(
                    &mut data.map,
                    self.settings.symmetry,
                    self.settings.brush_size,
                    drunk,
                );
                data.map.tiles[drunk_idx].surface = Surface::DownStairs;

                match rng.range(1, 5) {
                    1 => {
                        if drunk.x > 2 {
                            drunk.x -= 1;
                        }
                    }
                    2 => {
                        if drunk.x < data.map.width - 2 {
                            drunk.x += 1;
                        }
                    }
                    3 => {
                        if drunk.y > 2 {
                            drunk.y -= 1;
                        }
                    }
                    _ => {
                        if drunk.y < data.map.height - 2 {
                            drunk.y += 1;
                        }
                    }
                }
            }
            if did_something {
                data.take_snapshot();
            }

            digger_count += 1;
            for t in data.map.tiles.iter_mut() {
                if t.surface == Surface::DownStairs {
                    t.surface = Surface::Floor;
                }
            }
            floor_tile_count = data
                .map
                .tiles
                .iter()
                .filter(|a| a.surface == Surface::Floor)
                .count();
        }
    }
}

impl super::InitialMapBuilder for DrunkardsWalkBuilder {
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        self.build(data);
    }
}

impl super::MetaMapBuilder for DrunkardsWalkBuilder {
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        self.build(data);
    }
}
