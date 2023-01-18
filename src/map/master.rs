use super::{Map, OtherLevelPosition, Position};
use specs::prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct MasterMap {
    maps: HashMap<i32, Map>,
}

impl MasterMap {
    pub fn new() -> Self {
        Self {
            maps: HashMap::new(),
        }
    }

    pub fn store_map(&mut self, map: &Map) {
        self.maps.insert(map.depth, map.clone());
    }

    pub fn get_map(&self, depth: i32) -> Option<Map> {
        if !self.maps.contains_key(&depth) {
            return None;
        }

        Some(self.maps[&depth].clone())
    }
}

pub fn freeze_level_entities(ecs: &mut World) {
    let entities = ecs.entities();
    let mut positions = ecs.write_storage::<Position>();
    let mut other_level_positions = ecs.write_storage::<OtherLevelPosition>();
    let player_entity = ecs.fetch::<Entity>();
    let depth = ecs.fetch::<Map>().depth;

    let mut pos_to_delete: Vec<Entity> = Vec::new();
    for (entity, pos) in (&entities, &positions).join() {
        if entity == *player_entity {
            continue;
        }
        other_level_positions
            .insert(
                entity,
                OtherLevelPosition {
                    x: pos.x,
                    y: pos.y,
                    depth,
                },
            )
            .expect("Insert fail");
        pos_to_delete.push(entity);
    }

    for p in pos_to_delete.iter() {
        positions.remove(*p);
    }
}

pub fn unfreeze_level_entities(ecs: &mut World) {
    let entities = ecs.entities();
    let mut positions = ecs.write_storage::<Position>();
    let mut other_level_positions = ecs.write_storage::<OtherLevelPosition>();
    let player_entity = ecs.fetch::<Entity>();
    let depth = ecs.fetch::<Map>().depth;

    let mut pos_to_delete: Vec<Entity> = Vec::new();
    for (entity, pos) in (&entities, &other_level_positions).join() {
        if entity != *player_entity && pos.depth == depth {
            positions
                .insert(entity, Position { x: pos.x, y: pos.y })
                .expect("Insert fail");
            pos_to_delete.push(entity);
        }
    }

    for p in pos_to_delete.iter() {
        other_level_positions.remove(*p);
    }
}
