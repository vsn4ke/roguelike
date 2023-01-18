use super::super::{
    super::{colors::*, pathfinding::a_star::a_star_search},
    particle::ParticleBuilder,
    spatial::move_entity,
    Confusion, EntityMoved, Map, Monster, Position, RunState, Viewshed, WantsToMelee,
};

use bracket_lib::{
    prelude::Algorithm2D,
    terminal::{to_cp437, DistanceAlg, Point},
};
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        ReadExpect<'a, Entity>,
        ReadExpect<'a, RunState>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, WantsToMelee>,
        WriteStorage<'a, Confusion>,
        WriteExpect<'a, ParticleBuilder>,
        WriteStorage<'a, EntityMoved>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            map,
            player_pt,
            player_entity,
            runstate,
            entities,
            mut viewshed,
            monster,
            mut position,
            mut wants_to_melee,
            mut confused,
            mut particle_builder,
            mut entity_moved,
        ) = data;

        if *runstate != RunState::MonsterTurn {
            return;
        }

        for (_, entity, mut monster_position, mut viewshed) in
            (&monster, &entities, &mut position, &mut viewshed).join()
        {
            let mut can_act = true;
            let is_confused = confused.get_mut(entity);
            if let Some(i_am_confused) = is_confused {
                i_am_confused.duration -= 1;
                if i_am_confused.duration < 1 {
                    confused.remove(entity);
                }
                can_act = false;
                particle_builder.request(
                    monster_position.x,
                    monster_position.y,
                    c(GREEN4),
                    c(BLACK),
                    to_cp437('?'),
                    200.0,
                );
            }

            if !can_act {
                continue;
            }

            let distance = DistanceAlg::PythagorasSquared
                .distance2d(monster_position.into_point(), *player_pt);

            if distance < 4.0 {
                wants_to_melee
                    .insert(
                        entity,
                        WantsToMelee {
                            target: *player_entity,
                        },
                    )
                    .expect("Unable to insert attack");
            } else if viewshed.visible_tiles.contains(&*player_pt) {
                // Path to the player
                let path = a_star_search(
                    map.coord_to_index(monster_position.x, monster_position.y),
                    map.point2d_to_index(*player_pt),
                    &*map,
                );
                if path.success && path.steps.len() > 1 {
                    let moving_from = map.coord_to_index(monster_position.x, monster_position.y);
                    monster_position.x = path.steps[1] as i32 % map.width;
                    monster_position.y = path.steps[1] as i32 / map.width;
                    entity_moved
                        .insert(entity, EntityMoved {})
                        .expect("Unable to insert marker");
                    let moving_to = map.coord_to_index(monster_position.x, monster_position.y);
                    move_entity(entity, moving_from, moving_to);
                    viewshed.dirty = true;
                }
            }
        }
    }
}
