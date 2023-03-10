use crate::pathfinding::a_star::a_star_search;
use specs::prelude::*;

use super::{Chasing, EntityMoved, Map, MyTurn, Position, Viewshed};
use std::collections::HashMap;

pub struct ChaseAI {}

impl<'a> System<'a> for ChaseAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'a, MyTurn>,
        WriteStorage<'a, Chasing>,
        WriteStorage<'a, Position>,
        WriteExpect<'a, Map>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, EntityMoved>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut turns,
            mut chasing,
            mut positions,
            mut map,
            mut viewsheds,
            mut entity_moved,
            entities,
        ) = data;

        let mut targets: HashMap<Entity, (i32, i32)> = HashMap::new();
        let mut end_chase = Vec::<Entity>::new();
        for (entity, _, chasing) in (&entities, &turns, &chasing).join() {
            let target_pos = positions.get(chasing.target);

            if let Some(target_pos) = target_pos {
                targets.insert(entity, (target_pos.x, target_pos.y));
            } else {
                end_chase.push(entity);
            }
        }

        for done in end_chase.iter() {
            chasing.remove(*done);
        }
        end_chase.clear();

        let mut turn_done = Vec::<Entity>::new();
        for (entity, mut pos, _chase, mut viewshed, _) in
            (&entities, &mut positions, &chasing, &mut viewsheds, &turns).join()
        {
            turn_done.push(entity);
            let target_pos = targets[&entity];
            let path = a_star_search(
                map.coord_to_index(pos.x, pos.y) as i32,
                map.coord_to_index(target_pos.0, target_pos.1) as i32,
                &*map,
            );
            if path.success && path.steps.len() > 1 && path.steps.len() < 15 {
                let idx = map.coord_to_index(pos.x, pos.y);
                pos.x = path.steps[1] as i32 % map.width;
                pos.y = path.steps[1] as i32 / map.width;
                entity_moved
                    .insert(entity, EntityMoved {})
                    .expect("Unable to insert marker");
                let dest_idx = map.coord_to_index(pos.x, pos.y);

                map.move_entity(entity, idx, dest_idx);
                viewshed.dirty = true;
                turn_done.push(entity);
            } else {
                end_chase.push(entity);
            }
        }

        for done in end_chase.iter() {
            chasing.remove(*done);
        }
        for done in turn_done.iter() {
            turns.remove(*done);
        }
    }
}
