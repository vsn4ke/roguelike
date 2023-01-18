use super::{random_valid_points_finder, BuilderMap, MetaMapBuilder, X, Y};
use bracket_lib::terminal::Point;

pub struct AreaStartingPosition {
    x: X,
    y: Y,
}

impl AreaStartingPosition {
    #[allow(dead_code)]
    pub fn new((x, y): (X, Y)) -> Box<AreaStartingPosition> {
        Box::new(AreaStartingPosition { x, y })
    }
}

impl MetaMapBuilder for AreaStartingPosition {
    fn build_map(&mut self, data: &mut BuilderMap) {
        let starting_idx = random_valid_points_finder(&self.x, &self.y, data);

        data.starting_point = Some(Point::new(
            starting_idx as i32 % data.map.width,
            starting_idx as i32 / data.map.width,
        ));
    }
}
