use bracket_lib::prelude::{Algorithm2D, DijkstraMap};

use super::Surface;
pub struct DistantExit {}

impl DistantExit {
    pub fn new() -> Box<DistantExit> {
        Box::new(DistantExit {})
    }
}

impl super::MetaMapBuilder for DistantExit {
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

        let mut exit_tile = (0, 0.0f32);

        for (i, tile) in data.map.tiles.iter_mut().enumerate() {
            if tile.surface == Surface::Floor
                && dijkstra_map.map[i] != std::f32::MAX
                && dijkstra_map.map[i] > exit_tile.1
            {
                exit_tile.0 = i;
                exit_tile.1 = dijkstra_map.map[i];
            }
        }

        data.map.tiles[exit_tile.0].surface = Surface::DownStairs;
        data.take_snapshot();
    }
}
