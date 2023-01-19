use super::{
    super::{map_builders::level_builder, unit::Viewshed},
    spatial::set_size,
    Map, OtherLevelPosition, Position, Surface,
};
use bracket_lib::{prelude::Algorithm2D, terminal::Point};
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
    let map = ecs.fetch::<Map>();
    let depth = map.depth;
    set_size((map.width * map.height) as usize);

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

pub fn level_transition(ecs: &mut World, depth: i32, offset: i32) -> Option<Vec<Map>> {
    if ecs.read_resource::<MasterMap>().get_map(depth).is_some() {
        transition_to_existing_map(ecs, depth, offset);
        None
    } else {
        Some(transition_to_new_map(ecs, depth))
    }
}

pub fn transition_to_new_map(ecs: &mut World, depth: i32) -> Vec<Map> {
    let mut builder = level_builder(depth, 40, 40);
    builder.build_map();

    if depth > 0 {
        if let Some(pos) = &builder.data.starting_point {
            let idx = builder.data.map.point2d_to_index(*pos);
            builder.data.map.tiles[idx].surface = Surface::UpStairs;
        }
    }
    let history = builder.data.history.clone();

    let start;
    {
        *ecs.write_resource::<Map>() = builder.data.map.clone();
        start = *builder.data.starting_point.as_mut().unwrap();
    }

    builder.spawn_entities(ecs);
    *ecs.write_resource::<Point>() = start;

    let player_entity = ecs.fetch::<Entity>();

    if let Some(ppos) = ecs.write_storage::<Position>().get_mut(*player_entity) {
        ppos.x = start.x;
        ppos.y = start.y;
    }

    if let Some(vs) = ecs.write_storage::<Viewshed>().get_mut(*player_entity) {
        vs.dirty = true;
    }

    ecs.write_resource::<MasterMap>()
        .store_map(&builder.data.map);

    history
}

pub fn transition_to_existing_map(ecs: &mut World, depth: i32, offset: i32) {
    let map = ecs.write_resource::<MasterMap>().get_map(depth).unwrap();
    let player_entity = ecs.fetch::<Entity>();
    let stair = if offset < 0 {
        Surface::DownStairs
    } else {
        Surface::UpStairs
    };

    for (idx, tile) in map.tiles.iter().enumerate() {
        if tile.surface == stair {
            let mut player_pt = ecs.write_resource::<Point>();

            *player_pt = map.index_to_point2d(idx);

            if let Some(player_pos) = ecs.write_storage::<Position>().get_mut(*player_entity) {
                player_pos.x = player_pt.x;
                player_pos.y = player_pt.y;
            }
        }
    }

    *ecs.write_resource::<Map>() = map;

    if let Some(viewshed) = ecs.write_storage::<Viewshed>().get_mut(*player_entity) {
        viewshed.dirty = true;
    }
}
