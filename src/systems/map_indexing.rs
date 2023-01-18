use super::{
    spatial::{clear_spatial_map, index_entity, populate_blocked_from_map},
    Map, Position,
};
use ::specs::prelude::*;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, position, entities) = data;

        clear_spatial_map();
        populate_blocked_from_map(&map);

        for (entity, position) in (&entities, &position).join() {
            let idx = map.coord_to_index(position.x, position.y);

            index_entity(entity, idx);
        }
    }
}
