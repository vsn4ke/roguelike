use super::{random_valid_points_finder, Surface, X, Y};
use bracket_lib::{
    prelude::{a_star_search, Algorithm2D},
    random::RandomNumberGenerator,
    terminal::Point,
};

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

        let mut rng = RandomNumberGenerator::new();
        let stream_idx = path.steps[path.steps.len() * 4 / 5];

        let y = match rng.range(0, 2) {
            0 => Y::Top,
            _ => Y::Bottom,
        };

        let stair = random_valid_points_finder(&X::Right, &y, data);

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
