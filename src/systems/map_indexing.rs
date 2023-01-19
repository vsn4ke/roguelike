use super::{
    spatial::{clear_spatial_map, index_entity, populate_blocked_from_map},
    Map, Pools, Position,
};
use ::specs::prelude::*;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Pools>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, position, pools, entities) = data;

        clear_spatial_map();
        populate_blocked_from_map(&map);

        for (entity, position) in (&entities, &position).join() {
            let mut alive = true;
            if let Some(pools) = pools.get(entity) {
                if pools.hit_points.current < 1 {
                    alive = false;
                }
            }
            if alive {
                let idx = map.coord_to_index(position.x, position.y);

                index_entity(entity, idx);
            }
        }
    }
}
