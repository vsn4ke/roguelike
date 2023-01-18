use bracket_lib::{
    prelude::{CellularDistanceFunction, FastNoise, NoiseType},
    random::RandomNumberGenerator,
};

use super::Surface;
use std::collections::HashMap;
pub struct VoronoiSpawner {}

impl VoronoiSpawner {
    #[allow(dead_code)]
    pub fn new() -> Box<VoronoiSpawner> {
        Box::new(VoronoiSpawner {})
    }
}

impl super::MetaMapBuilder for VoronoiSpawner {
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        let mut rng = RandomNumberGenerator::new();
        let mut noise_areas: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut noise = FastNoise::seeded(rng.range(1, 65536) as u64);
        noise.set_noise_type(NoiseType::Cellular);
        noise.set_frequency(0.08);
        noise.set_cellular_distance_function(CellularDistanceFunction::Manhattan);

        for y in 1..data.map.height - 1 {
            for x in 1..data.map.width - 1 {
                let idx = data.map.coord_to_index(x, y);
                if data.map.tiles[idx].surface == Surface::Floor {
                    let cell_value_f = noise.get_noise(x as f32, y as f32) * 10240.0;
                    let cell_value = cell_value_f as i32;

                    if let std::collections::hash_map::Entry::Vacant(e) =
                        noise_areas.entry(cell_value)
                    {
                        e.insert(vec![idx]);
                    } else {
                        noise_areas.get_mut(&cell_value).unwrap().push(idx);
                    }
                }
            }
        }

        for area in noise_areas.iter() {
            super::spawner::spawn_in_region(area.1, data.map.depth, &mut data.spawn_list);
        }
    }
}
