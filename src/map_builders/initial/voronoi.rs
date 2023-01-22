use bracket_lib::{
    prelude::Algorithm2D,
    terminal::{DistanceAlg, Point},
};

use super::{RandomGen, Surface};

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy)]
pub enum DistanceAlgorithm {
    Pythagoras,
    Manhattan,
    Chebyshev,
}

pub struct VoronoiCellBuilder {
    distance_algorithm: DistanceAlgorithm,
}

impl VoronoiCellBuilder {
    #[allow(dead_code)]
    fn new(distance_algorithm: DistanceAlgorithm) -> VoronoiCellBuilder {
        VoronoiCellBuilder { distance_algorithm }
    }

    #[allow(dead_code)]
    pub fn chebyshev() -> Box<VoronoiCellBuilder> {
        Box::new(VoronoiCellBuilder::new(DistanceAlgorithm::Chebyshev))
    }

    #[allow(dead_code)]
    pub fn pythagoras() -> Box<VoronoiCellBuilder> {
        Box::new(VoronoiCellBuilder::new(DistanceAlgorithm::Pythagoras))
    }

    #[allow(dead_code)]
    pub fn manhattan() -> Box<VoronoiCellBuilder> {
        Box::new(VoronoiCellBuilder::new(DistanceAlgorithm::Manhattan))
    }
}

impl super::InitialMapBuilder for VoronoiCellBuilder {
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        let mut rng = RandomGen::default();
        let starting_point = Point::new(data.map.width / 2, data.map.height / 2);

        let start_idx = data.map.point2d_to_index(starting_point);
        data.map.tiles[start_idx].surface = Surface::Floor;
        data.take_snapshot();

        let mut voronoi_seeds: Vec<(usize, Point)> = Vec::new();
        let n_seeds = 64;

        while voronoi_seeds.len() < n_seeds {
            let v = Point::new(rng.range(1, data.map.width), rng.range(1, data.map.height));
            let idx = data.map.point2d_to_index(v);
            let candidate = (idx, v);
            if !voronoi_seeds.contains(&candidate) {
                voronoi_seeds.push(candidate);
            }
        }

        let mut voronoi_distance = vec![(0, 0.0f32); n_seeds];
        let mut voronoi_membership: Vec<i32> = vec![0; (data.map.width * data.map.height) as usize];
        for (i, vid) in voronoi_membership.iter_mut().enumerate() {
            let p = Point::new(i as i32 % data.map.width, i as i32 / data.map.width);
            for (seed, pos) in voronoi_seeds.iter().enumerate() {
                let distance = match self.distance_algorithm {
                    DistanceAlgorithm::Chebyshev => DistanceAlg::Chebyshev.distance2d(p, pos.1),
                    DistanceAlgorithm::Manhattan => DistanceAlg::Manhattan.distance2d(p, pos.1),
                    DistanceAlgorithm::Pythagoras => {
                        DistanceAlg::PythagorasSquared.distance2d(p, pos.1)
                    }
                };
                DistanceAlg::PythagorasSquared.distance2d(p, pos.1);
                voronoi_distance[seed] = (seed, distance);
            }

            voronoi_distance.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            *vid = voronoi_distance[0].0 as i32;
        }

        for y in 1..data.map.height - 1 {
            for x in 1..data.map.width - 1 {
                let mut neighbors = 0;
                let idx = data.map.coord_to_index(x, y);
                let my_seed = voronoi_membership[idx];
                if voronoi_membership[data.map.coord_to_index(x - 1, y)] != my_seed {
                    neighbors += 1;
                }
                if voronoi_membership[data.map.coord_to_index(x + 1, y)] != my_seed {
                    neighbors += 1;
                }
                if voronoi_membership[data.map.coord_to_index(x, y - 1)] != my_seed {
                    neighbors += 1;
                }
                if voronoi_membership[data.map.coord_to_index(x, y + 1)] != my_seed {
                    neighbors += 1;
                }
                if neighbors < 2 {
                    data.map.tiles[idx].surface = Surface::Floor;
                }
            }
            if y % 2 == 0 {
                data.take_snapshot();
            }
        }
    }
}
