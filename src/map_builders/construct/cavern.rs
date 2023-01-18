use bracket_lib::random::RandomNumberGenerator;

use super::{
    AreaStartingPosition, BuilderChain, CullUnreachable, DistantExit, DrunkardsWalkBuilder, Map,
    Surface, VoronoiSpawner, X, Y,
};

pub fn cavern_builder(depth: i32, width: i32, height: i32) -> BuilderChain {
    let mut builder = BuilderChain::new(depth, width, height, "The Ominous Cavern");
    builder
        .start_with(DrunkardsWalkBuilder::winding_passages())
        .with(AreaStartingPosition::new((X::Center, Y::Center)))
        .with(CullUnreachable::new())
        .with(AreaStartingPosition::new((X::Left, Y::Center)))
        .with(CavernDecorator::new())
        .with(VoronoiSpawner::new())
        .with(DistantExit::new());
    builder
}

pub struct CavernDecorator {}

impl CavernDecorator {
    #[allow(dead_code)]
    pub fn new() -> Box<CavernDecorator> {
        Box::new(CavernDecorator {})
    }
}

impl super::MetaMapBuilder for CavernDecorator {
    fn build_map(&mut self, data: &mut crate::map_builders::BuilderMap) {
        let mut rng = RandomNumberGenerator::new();
        let map = data.map.clone();

        for (idx, tile) in data.map.tiles.iter_mut().enumerate() {
            tile.surface = match tile.surface {
                Surface::Floor => match rng.range(0, 100) {
                    0..=60 => Surface::Gravel,
                    61..=66 => Surface::ShallowWater,
                    _ => Surface::Floor,
                },
                Surface::Wall => match count_neighbors(idx, &map) {
                    1 => match rng.range(0, 4) {
                        0 => Surface::Stalactite,
                        1 => Surface::Stalagmite,
                        _ => Surface::Wall,
                    },
                    2 => Surface::DeepWater,
                    _ => Surface::Wall,
                },
                _ => tile.surface,
            }
        }
        data.take_snapshot();
        data.map.outdoors = false;
    }
}

fn count_neighbors(idx: usize, map: &Map) -> i32 {
    let mut neighbors = 0;
    let x = idx as i32 % map.width;
    let y = idx as i32 / map.width;
    if x > 0 && map.tiles[idx - 1].surface == Surface::Wall {
        neighbors += 1;
    }
    if x < map.width - 2 && map.tiles[idx + 1].surface == Surface::Wall {
        neighbors += 1;
    }
    if y > 0 && map.tiles[idx - map.width as usize].surface == Surface::Wall {
        neighbors += 1;
    }
    if y < map.height - 2 && map.tiles[idx + map.width as usize].surface == Surface::Wall {
        neighbors += 1;
    }

    neighbors
}
