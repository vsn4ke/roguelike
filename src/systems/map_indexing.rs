use super::{tiles::is_tile_walkable, BlocksTile, Map, Pools, Position};
use bracket_lib::{prelude::Algorithm2D, terminal::Point};
use ::specs::prelude::*;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Pools>,
        Entities<'a>,
        ReadStorage<'a, BlocksTile>,
        ReadExpect<'a, Entity>,
        ReadExpect<'a, Point>,

    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, pools, entities, block, player_entity, ppos) = data;

        for (i, tile) in map.tiles.clone().iter().enumerate() {
            map.tiles[i].content.clear();
            map.tiles[i].block_movement = !is_tile_walkable(tile.surface);
        }

        for (entity, position, _) in (&entities, &position, &block).join() {
            let mut alive = true;
            let idx = map.coord_to_index(position.x, position.y);

            if let Some(pools) = pools.get(entity) {
                if pools.hit_points.current < 1 {
                    alive = false;
                }
            }
            if alive {
                map.tiles[idx].content.push(entity);
            }

            map.blocks_movement(idx, true);
        }

        let idx = map.point2d_to_index(*ppos);
        map.tiles[idx].content.push(*player_entity);

    }
}
