use super::super::{
    spatial::{is_blocked, move_entity},
    Bystander, EntityMoved, Log, Map, Name, Position, Quips, RunState, Viewshed,
};
use bracket_lib::{prelude::Algorithm2D, random::RandomNumberGenerator, terminal::Point};
use specs::prelude::*;

pub struct BystanderAI {}

impl<'a> System<'a> for BystanderAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, RunState>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Bystander>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, EntityMoved>,
        WriteStorage<'a, Quips>,
        ReadStorage<'a, Name>,
        ReadExpect<'a, Point>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            map,
            runstate,
            entities,
            mut viewshed,
            bystander,
            mut position,
            mut entity_moved,
            mut quips,
            sources,
            player_pos,
        ) = data;

        if *runstate != RunState::MonsterTurn {
            return;
        }

        for (entity, mut viewshed, _, mut pos) in
            (&entities, &mut viewshed, &bystander, &mut position).join()
        {
            if !viewshed.visible_tiles.contains(&player_pos) {
                continue;
            }

            let mut rng = RandomNumberGenerator::new();
            if let Some(quip) = quips.get_mut(entity) {
                if let Some(quip_index) = rng.random_slice_index(&quip.available) {
                    if rng.range(0, 8) == 0 {
                        Log::new()
                            .npc(&sources.get(entity).unwrap().name)
                            .append("says \"")
                            .item(&quip.available[quip_index])
                            .append("\"")
                            .build();

                        quip.available.remove(quip_index);
                    }
                }
            }

            let mut p = pos.into_point();

            match rng.range(1, 6) {
                1 => p.x -= 1,
                2 => p.x += 1,
                3 => p.y -= 1,
                4 => p.y += 1,
                _ => {}
            }

            let dest_idx = map.point2d_to_index(p);

            if !map.in_bounds(p) || is_blocked(dest_idx) {
                continue;
            }

            let idx = map.coord_to_index(pos.x, pos.y);

            move_entity(entity, idx, dest_idx);
            pos.x = p.x;
            pos.y = p.y;
            entity_moved
                .insert(entity, EntityMoved {})
                .expect("Unable to insert marker");
            viewshed.dirty = true;
        }
    }
}
