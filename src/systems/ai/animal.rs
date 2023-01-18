use super::super::{
    spatial::{get_content, is_blocked, move_entity},
    Carnivore, EntityMoved, Herbivore, Item, Map, Position, RunState, Viewshed, WantsToMelee,
};
use bracket_lib::{
    prelude::{Algorithm2D, DijkstraMap},
    terminal::{DistanceAlg, Point},
};
use specs::prelude::*;

pub struct AnimalAI {}

impl<'a> System<'a> for AnimalAI {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Entity>,
        ReadExpect<'a, RunState>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Herbivore>,
        ReadStorage<'a, Carnivore>,
        ReadStorage<'a, Item>,
        WriteStorage<'a, WantsToMelee>,
        WriteStorage<'a, EntityMoved>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            map,
            player_entity,
            runstate,
            entities,
            mut viewshed,
            herbivore,
            carnivore,
            item,
            mut wants_to_melee,
            mut entity_moved,
            mut position,
        ) = data;

        if *runstate != RunState::MonsterTurn {
            return;
        }

        let mut ppos = Point::new(0, 0);
        for (entity, pos) in (&entities, &position).join() {
            if entity == *player_entity {
                ppos.x = pos.x;
                ppos.y = pos.y;
            }
        }

        for (entity, mut viewshed, _, mut pos) in
            (&entities, &mut viewshed, &herbivore, &mut position).join()
        {
            if DistanceAlg::PythagorasSquared.distance2d(ppos, pos.into_point()) > 100.0 {
                continue;
            }

            let mut run_away_from: Vec<usize> = Vec::new();
            for tiles in viewshed.visible_tiles.iter() {
                let idx = map.point2d_to_index(*tiles);
                for other_entity in get_content(idx).iter() {
                    if item.get(*other_entity).is_none() {
                        run_away_from.push(idx);
                    }
                }
            }
            if run_away_from.is_empty() {
                continue;
            }

            let idx = map.coord_to_index(pos.x, pos.y);
            map.populate_blocked();

            let flee_map = DijkstraMap::new(map.width, map.height, &run_away_from, &*map, 100.0);
            if let Some(flee_target) = DijkstraMap::find_highest_exit(&flee_map, idx, &*map) {
                if is_blocked(flee_target) {
                    continue;
                }
                move_entity(entity, idx, flee_target);
                viewshed.dirty = true;
                pos.x = flee_target as i32 % map.width;
                pos.y = flee_target as i32 / map.width;
                entity_moved
                    .insert(entity, EntityMoved {})
                    .expect("Unable to insert entity");
            }
        }
        for (entity, mut viewshed, _, mut pos) in
            (&entities, &mut viewshed, &carnivore, &mut position).join()
        {
            let pos_pt = pos.into_point();
            if DistanceAlg::PythagorasSquared.distance2d(ppos, pos_pt) > 100.0 {
                continue;
            }

            let mut run_towards: Vec<usize> = Vec::new();
            let mut attacked = false;
            for tiles in viewshed.visible_tiles.iter() {
                let idx = map.point2d_to_index(*tiles);
                for other_entity in get_content(idx).iter() {
                    if herbivore.get(*other_entity).is_some() || *other_entity == *player_entity {
                        let distance = DistanceAlg::PythagorasSquared.distance2d(pos_pt, *tiles);
                        if distance < 4.0 {
                            wants_to_melee
                                .insert(
                                    entity,
                                    WantsToMelee {
                                        target: *other_entity,
                                    },
                                )
                                .expect("Unable to insert attack");
                            attacked = true;
                        } else {
                            run_towards.push(idx);
                        }
                    }
                }
            }

            if run_towards.is_empty() || attacked {
                continue;
            }
            let idx = map.coord_to_index(pos.x, pos.y);
            map.populate_blocked();
            let chase_map = DijkstraMap::new(map.width, map.height, &run_towards, &*map, 100.0);
            if let Some(chase_target) = DijkstraMap::find_lowest_exit(&chase_map, idx, &*map) {
                if is_blocked(chase_target) {
                    continue;
                }
                move_entity(entity, idx, chase_target);
                viewshed.dirty = true;
                pos.x = chase_target as i32 % map.width;
                pos.y = chase_target as i32 / map.width;
                entity_moved
                    .insert(entity, EntityMoved {})
                    .expect("Unable to insert marker");
            }
        }
    }
}
