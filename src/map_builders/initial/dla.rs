use bracket_lib::{
    prelude::Algorithm2D,
    terminal::{line2d, LineAlg, Point},
};

use super::{
    super::meta::paint::{paint, Symmetry},
    RandomGen, Surface,
};

#[derive(PartialEq, Copy, Clone)]
pub enum DLAAlgorithm {
    WalkInwards,
    WalkOutwards,
    CentralAttractor,
}

pub struct DLABuilder {
    algorithm: DLAAlgorithm,
    brush_size: i32,
    symmetry: Symmetry,
    floor_percent: f32,
}

impl DLABuilder {
    #[allow(dead_code)]
    fn new() -> Box<DLABuilder> {
        Box::new(DLABuilder {
            algorithm: DLAAlgorithm::WalkInwards,
            brush_size: 2,
            symmetry: Symmetry::None,
            floor_percent: 0.25,
        })
    }

    #[allow(dead_code)]
    pub fn walk_inwards() -> Box<DLABuilder> {
        Box::new(DLABuilder {
            algorithm: DLAAlgorithm::WalkInwards,
            brush_size: 1,
            symmetry: Symmetry::None,
            floor_percent: 0.25,
        })
    }

    #[allow(dead_code)]
    pub fn walk_outwards() -> Box<DLABuilder> {
        Box::new(DLABuilder {
            algorithm: DLAAlgorithm::WalkOutwards,
            brush_size: 2,
            symmetry: Symmetry::None,
            floor_percent: 0.25,
        })
    }

    #[allow(dead_code)]
    pub fn central_attractor() -> Box<DLABuilder> {
        Box::new(DLABuilder {
            algorithm: DLAAlgorithm::CentralAttractor,
            brush_size: 2,
            symmetry: Symmetry::None,
            floor_percent: 0.25,
        })
    }

    #[allow(dead_code)]
    pub fn insectoid() -> Box<DLABuilder> {
        Box::new(DLABuilder {
            algorithm: DLAAlgorithm::CentralAttractor,
            brush_size: 2,
            symmetry: Symmetry::Horizontal,
            floor_percent: 0.25,
        })
    }

    #[allow(dead_code)]
    pub fn heavy_erosion() -> Box<DLABuilder> {
        Box::new(DLABuilder {
            algorithm: DLAAlgorithm::WalkInwards,
            brush_size: 2,
            symmetry: Symmetry::None,
            floor_percent: 0.35,
        })
    }

    fn build(&mut self, data: &mut super::BuilderMap) {
        let mut rng = RandomGen::default();
        let starting_position = Point::new(data.map.width / 2, data.map.height / 2);
        let start_idx = data.map.point2d_to_index(starting_position);
        data.take_snapshot();

        data.map.tiles[start_idx - 1].surface = Surface::Floor;
        data.map.tiles[start_idx + 1].surface = Surface::Floor;
        data.map.tiles[start_idx - data.map.width as usize].surface = Surface::Floor;
        data.map.tiles[start_idx + data.map.width as usize].surface = Surface::Floor;

        let total_tiles = data.map.width * data.map.height;
        let desired_floor = (self.floor_percent * total_tiles as f32) as usize;
        let mut floor_tile_count = data
            .map
            .tiles
            .iter()
            .filter(|a| a.surface == Surface::Floor)
            .count();

        let mut i = 0;
        while floor_tile_count < desired_floor {
            match self.algorithm {
                DLAAlgorithm::WalkInwards => {
                    let mut digger = Point::new(
                        rng.range(2, data.map.width - 1),
                        rng.range(2, data.map.height - 1),
                    );
                    let mut prev = digger;
                    let mut digger_idx = data.map.point2d_to_index(digger);

                    while data.map.tiles[digger_idx].surface == Surface::Wall {
                        prev = digger;

                        match rng.range(1, 5) {
                            1 => {
                                if digger.x > 2 {
                                    digger.x -= 1;
                                }
                            }
                            2 => {
                                if digger.x < data.map.width - 2 {
                                    digger.x += 1;
                                }
                            }
                            3 => {
                                if digger.y > 2 {
                                    digger.y -= 1;
                                }
                            }
                            _ => {
                                if digger.y < data.map.height - 2 {
                                    digger.y += 1
                                }
                            }
                        }

                        digger_idx = data.map.point2d_to_index(digger);
                    }

                    paint(&mut data.map, self.symmetry, self.brush_size, prev);
                }
                DLAAlgorithm::WalkOutwards => {
                    let mut digger = starting_position;
                    let mut digger_idx = data.map.point2d_to_index(digger);
                    while data.map.tiles[digger_idx].surface == Surface::Floor {
                        match rng.range(1, 5) {
                            1 => {
                                if digger.x > 2 {
                                    digger.x -= 1;
                                }
                            }
                            2 => {
                                if digger.x < data.map.width - 2 {
                                    digger.x += 1;
                                }
                            }
                            3 => {
                                if digger.y > 2 {
                                    digger.y -= 1;
                                }
                            }
                            _ => {
                                if digger.y < data.map.height - 2 {
                                    digger.y += 1;
                                }
                            }
                        }
                        digger_idx = data.map.point2d_to_index(digger);
                    }
                    paint(&mut data.map, self.symmetry, self.brush_size, digger);
                }
                DLAAlgorithm::CentralAttractor => {
                    let mut digger = Point::new(
                        rng.range(1, data.map.width - 1),
                        rng.range(1, data.map.height - 1),
                    );
                    let mut prev = digger;
                    let mut digger_idx = data.map.point2d_to_index(digger);

                    let mut path = line2d(LineAlg::Bresenham, digger, starting_position);

                    while data.map.tiles[digger_idx].surface == Surface::Wall && !path.is_empty() {
                        prev = digger;
                        digger = path[0];
                        path.remove(0);
                        digger_idx = data.map.point2d_to_index(digger);
                    }
                    paint(&mut data.map, self.symmetry, self.brush_size, prev);
                }
            }
            if i % 50 == 0 {
                data.take_snapshot();
                i = 0;
            }

            i += 1;

            floor_tile_count = data
                .map
                .tiles
                .iter()
                .filter(|a| a.surface == Surface::Floor)
                .count();
        }
    }
}

impl super::InitialMapBuilder for DLABuilder {
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        self.build(data);
    }
}

impl super::MetaMapBuilder for DLABuilder {
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        self.build(data);
    }
}
