use crate::pathfinding::a_star::a_star_search;
use specs::prelude::*;

use super::{EntityMoved, Map, MyTurn, Position, Viewshed, WantsToApproach};

pub struct ApproachAI {}

impl<'a> System<'a> for ApproachAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'a, MyTurn>,
        WriteStorage<'a, WantsToApproach>,
        WriteStorage<'a, Position>,
        WriteExpect<'a, Map>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, EntityMoved>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut turns,
            mut want_approach,
            mut positions,
            mut map,
            mut viewsheds,
            mut entity_moved,
            entities,
        ) = data;

        let mut turn_done = Vec::<Entity>::new();
        for (entity, mut pos, approach, mut viewshed, _) in (
            &entities,
            &mut positions,
            &want_approach,
            &mut viewsheds,
            &turns,
        )
            .join()
        {
            turn_done.push(entity);
            let path = a_star_search(
                map.coord_to_index(pos.x, pos.y),
                map.coord_to_index(approach.idx % map.width, approach.idx / map.width),
                &*map,
            );
            if path.success && path.steps.len() > 1 {
                let idx = map.coord_to_index(pos.x, pos.y);
                pos.x = path.steps[1] as i32 % map.width;
                pos.y = path.steps[1] as i32 / map.width;
                entity_moved
                    .insert(entity, EntityMoved {})
                    .expect("Unable to insert marker");
                let new_idx = map.coord_to_index(pos.x, pos.y);
                map.move_entity(entity, idx, new_idx);
                viewshed.dirty = true;
            }
        }

        want_approach.clear();

        for done in turn_done.iter() {
            turns.remove(*done);
        }
    }
}
