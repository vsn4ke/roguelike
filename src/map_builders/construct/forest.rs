use super::{
    random_valid_points_finder, AreaStartingPosition, BuilderChain, CellularAutomataBuilder,
    CullUnreachable, RandomGen, Surface, VoronoiSpawner, X, Y,
};
use bracket_lib::{
    prelude::{a_star_search, Algorithm2D},
    terminal::Point,
};

pub fn forest_builder(depth: i32, width: i32, height: i32) -> BuilderChain {
    let mut builder = BuilderChain::new(depth, width, height, "The Deep Dark Forest");
    builder
        .start_with(CellularAutomataBuilder::new())
        .with(AreaStartingPosition::new((X::Center, Y::Center)))
        .with(CullUnreachable::new())
        .with(AreaStartingPosition::new((X::Left, Y::Center)))
        .with(VoronoiSpawner::new())
        .with(ForestRoad::new());

    builder
}

pub struct ForestRoad {}

impl ForestRoad {
    #[allow(dead_code)]
    pub fn new() -> Box<ForestRoad> {
        Box::new(ForestRoad {})
    }
}

impl super::MetaMapBuilder for ForestRoad {
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        let start_idx = data.map.point2d_to_index(data.starting_point.unwrap());
        let end_of_road = random_valid_points_finder(&X::Right, &Y::Center, data);

        data.map.populate_blocked();
        let path = a_star_search(start_idx, end_of_road, &data.map);

        for idx in path.steps.iter() {
            let pt = data.map.index_to_point2d(*idx);
            paint_road(data, pt.x, pt.y);
            paint_road(data, pt.x - 1, pt.y);
            paint_road(data, pt.x + 1, pt.y);
            paint_road(data, pt.x, pt.y - 1);
            paint_road(data, pt.x, pt.y + 1);
        }
        data.take_snapshot();

        let mut rng = RandomGen::default();

        let stream_idx = if !path.steps.is_empty() {
            path.steps[path.steps.len() * 4 / 5]
        } else {
            random_valid_points_finder(&X::Right, &Y::Center, data)
        };

        let stair = random_valid_points_finder(
            &X::Right,
            &match rng.range(0, 2) {
                0 => Y::Top,
                _ => Y::Bottom,
            },
            data,
        );

        let stream = a_star_search(stair, stream_idx, &data.map);

        for idx in stream.steps.iter() {
            paint_water(data, *idx);
            paint_water(data, *idx + 1);
        }

        data.map.tiles[stair].surface = Surface::DownStairs;
        data.take_snapshot();
    }
}
fn paint_road(data: &mut super::BuilderMap, x: i32, y: i32) {
    let pt = Point::new(x, y);
    if !data.map.in_bounds(pt) {
        return;
    }

    let idx = data.map.point2d_to_index(pt);
    if data.map.tiles[idx].surface != Surface::DownStairs {
        data.map.tiles[idx].surface = Surface::Road;
    }
}

fn paint_water(data: &mut super::BuilderMap, idx: usize) {
    if data.map.tiles[idx].surface == Surface::Floor {
        data.map.tiles[idx].surface = Surface::ShallowWater;
    }
}
