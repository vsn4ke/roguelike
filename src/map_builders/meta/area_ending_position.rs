use super::{random_valid_points_finder, BuilderMap, MetaMapBuilder, Surface, X, Y};

pub struct AreaEndingPosition {
    x: X,
    y: Y,
}

impl AreaEndingPosition {
    #[allow(dead_code)]
    pub fn new((x, y): (X, Y)) -> Box<AreaEndingPosition> {
        Box::new(AreaEndingPosition { x, y })
    }
}

impl MetaMapBuilder for AreaEndingPosition {
    fn build_map(&mut self, data: &mut BuilderMap) {
        let idx = random_valid_points_finder(&self.x, &self.y, data);

        data.map.tiles[idx].surface = Surface::DownStairs;
    }
}
