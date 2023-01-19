use lazy_static::lazy_static;
use specs::prelude::*;
use std::sync::Mutex;

use super::{is_tile_walkable, Map};

#[derive(Clone)]
pub struct TileSpatial {
    pub block_movement: bool,
    pub content: Vec<Entity>,
}

struct SpatialMap {
    tiles: Vec<TileSpatial>,
}

impl SpatialMap {
    fn new() -> Self {
        Self { tiles: Vec::new() }
    }
}

lazy_static! {
    static ref SPATIAL_MAP: Mutex<SpatialMap> = Mutex::new(SpatialMap::new());
}

pub fn set_size(map_tile_count: usize) {
    let mut lock = SPATIAL_MAP.lock().unwrap();
    lock.tiles = vec![
        TileSpatial {
            block_movement: false,
            content: Vec::new(),
        };
        map_tile_count
    ];
}

pub fn clear_spatial_map() {
    let mut lock = SPATIAL_MAP.lock().unwrap();
    for tile in lock.tiles.iter_mut() {
        tile.content.clear();
        tile.block_movement = true;
    }
}

pub fn populate_blocked_from_map(map: &Map) {
    let mut lock = SPATIAL_MAP.lock().unwrap();

    for (i, tile) in map.tiles.iter().enumerate() {
        lock.tiles[i].block_movement = !is_tile_walkable(tile.surface);
    }
}

pub fn index_entity(entity: Entity, idx: usize) {
    let mut lock = SPATIAL_MAP.lock().unwrap();
    lock.tiles[idx].content.push(entity);
}

pub fn is_blocked(idx: usize) -> bool {
    SPATIAL_MAP.lock().unwrap().tiles[idx].block_movement
}

pub fn get_content(idx: usize) -> Vec<Entity> {
    SPATIAL_MAP.lock().unwrap().tiles[idx].content.clone()
}

pub fn move_entity(entity: Entity, moving_from: usize, moving_to: usize) {
    let mut lock = SPATIAL_MAP.lock().unwrap();
    for (i, e) in lock.tiles[moving_from].content.clone().iter().enumerate() {
        if *e == entity {
            lock.tiles[moving_from].content.remove(i);
            lock.tiles[moving_from].block_movement = false;
        }
    }
    lock.tiles[moving_to].content.push(entity);
    lock.tiles[moving_to].block_movement = true;
}
