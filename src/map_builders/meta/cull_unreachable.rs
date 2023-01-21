use bracket_lib::prelude::{Algorithm2D, DijkstraMap};

use super::Surface;
pub struct CullUnreachable {}

impl CullUnreachable {
    #[allow(dead_code)]
    pub fn new() -> Box<CullUnreachable> {
        Box::new(CullUnreachable {})
    }
}

impl super::MetaMapBuilder for CullUnreachable {
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        let idx = data
            .map
            .point2d_to_index(*data.starting_point.as_ref().unwrap());

        data.map.populate_blocked();      
        let map_starts: Vec<usize> = vec![idx];

        let dijkstra_map = DijkstraMap::new(
            data.map.width as usize,
            data.map.height as usize,
            &map_starts,
            &data.map,
            1000.0,
        );
        for (i, tile) in data.map.tiles.iter_mut().enumerate() {                        
            if tile.surface == Surface::Floor && dijkstra_map.map[i] == std::f32::MAX {
                tile.surface = Surface::Wall;
            }
        }
    }
}
