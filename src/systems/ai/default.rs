use crate::{map::tiles::is_tile_walkable, pathfinding::a_star::a_star_search};

use super::{EntityMoved, Map, Movement, MovementMode, MyTurn, Position, RandomGen, Viewshed};
use bracket_lib::prelude::Algorithm2D;
use specs::prelude::*;

pub struct DefaultMoveAI {}

impl<'a> System<'a> for DefaultMoveAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'a, MyTurn>,
        WriteStorage<'a, MovementMode>,
        WriteStorage<'a, Position>,
        WriteExpect<'a, Map>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, EntityMoved>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut turns,
            mut move_mode,
            mut positions,
            mut map,
            mut viewsheds,
            mut entity_moved,
            entities,
        ) = data;

        let mut turn_done = Vec::<Entity>::new();
        let mut rng = RandomGen::default();
        for (entity, mut pos, mut movement, mut viewshed, _) in (
            &entities,
            &mut positions,
            &mut move_mode,
            &mut viewsheds,
            &turns,
        )
            .join()
        {
            turn_done.push(entity);

            match &mut movement.mode {
                Movement::Static => {}
                Movement::Random => {
                    let mut pt = pos.into_point();
                    match rng.range(0, 5) {
                        0 => pt.x -= 1,
                        1 => pt.x += 1,
                        2 => pt.y -= 1,
                        3 => pt.y += 1,
                        _ => {}
                    }

                    let dest_idx = map.point2d_to_index(pt);

                    if !map.in_bounds(pt) || map.is_blocked(dest_idx) {
                        return;
                    }

                    let idx = map.point2d_to_index(pos.into_point());
                    pos.x = pt.x;
                    pos.y = pt.y;

                    entity_moved
                        .insert(entity, EntityMoved {})
                        .expect("Unable to insert marker");
                    map.move_entity(entity, idx, dest_idx);
                    viewshed.dirty = true;
                }
                Movement::Waypoint { path } => {
                    if let Some(path) = path {
                        let idx = map.coord_to_index(pos.x, pos.y);
                        if path.len() > 1 {
                            if !map.is_blocked(path[1]) {
                                pos.x = path[1] as i32 % map.width;
                                pos.y = path[1] as i32 / map.width;
                                entity_moved
                                    .insert(entity, EntityMoved {})
                                    .expect("Unable to insert marker");
                                let dest_idx = map.coord_to_index(pos.x, pos.y);
                                map.move_entity(entity, idx, dest_idx);
                                viewshed.dirty = true;
                                path.remove(0);
                            }
                        } else {
                            movement.mode = Movement::Waypoint { path: None };
                        }
                    } else {
                        let target_x = rng.range(0, map.width - 1);
                        let target_y = rng.range(0, map.height - 1);
                        let idx = map.coord_to_index(target_x, target_y);

                        if is_tile_walkable(map.tiles[idx].surface) {
                            let path = a_star_search(
                                map.coord_to_index(pos.x, pos.y) as i32,
                                map.coord_to_index(target_x, target_y) as i32,
                                &*map,
                            );
                            if path.success && path.steps.len() > 1 {
                                movement.mode = Movement::Waypoint {
                                    path: Some(path.steps),
                                };
                            }
                        }
                    }
                }
            }
        }

        for done in turn_done.iter() {
            turns.remove(*done);
        }
    }
}
