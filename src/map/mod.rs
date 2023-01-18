pub mod master;
pub mod spatial;
pub mod themes;
pub mod tiles;

use master::MasterMap;
use tiles::{is_tile_opaque, is_tile_walkable, Surface};

use super::{map_builders::level_builder, unit::Viewshed, OtherLevelPosition, Position};

use bracket_lib::{
    prelude::{Algorithm2D, BaseMap, SmallVec},
    terminal::{DistanceAlg, Point, Rect},
};

use specs::prelude::*;

#[derive(Clone)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub tiles: Vec<Tile>,
    pub rooms: Vec<Rect>,
    pub name: String,
}

#[derive(Clone, Default)]
pub struct Tile {
    pub surface: Surface,
    pub revealed: bool,
    pub visible: bool,
    pub block_visibility: bool,
    pub bloodstains: bool,
}

impl Tile {
    pub fn new() -> Self {
        Self {
            surface: Surface::Wall,
            revealed: false,
            visible: false,
            block_visibility: false,
            bloodstains: false,
        }
    }
}

impl Map {
    pub fn new<S: ToString>(depth: i32, width: i32, height: i32, name: S) -> Map {
        let tiles_count = (width * height) as usize;
        spatial::set_size(tiles_count);
        Map {
            width,
            height,
            depth,
            rooms: Vec::new(),
            name: name.to_string(),
            tiles: vec![Tile::new(); tiles_count],
        }
    }
    pub fn coord_to_index(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    fn is_exit_valid(&self, idx: usize) -> bool {
        self.in_bounds(self.index_to_point2d(idx)) && !spatial::is_blocked(idx)
    }

    pub fn populate_blocked(&self) {
        spatial::populate_blocked_from_map(self);
    }

    pub fn clear_content(&self) {
        spatial::clear_spatial_map();
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        pos.x > 0 && pos.x < self.width && pos.y > 0 && pos.y < self.height
    }
}

impl BaseMap for Map {
    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::PythagorasSquared
            .distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }

    fn is_opaque(&self, idx: usize) -> bool {
        is_tile_opaque(self.tiles[idx].surface) || self.tiles[idx].block_visibility
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let w = self.width as usize;

        if !self.in_bounds(self.index_to_point2d(idx)) {
            return exits;
        }

        let mut to_try = [idx - 1, idx + 1, idx - w, idx + w];

        for t in to_try.iter() {
            if self.is_exit_valid(*t) {
                exits.push((*t, 1.0));
            }
        }

        to_try = [idx - 1 - w, idx + 1 - w, idx - w + 1, idx + w - 1];

        for t in to_try.iter() {
            if self.is_exit_valid(*t) {
                exits.push((*t, 2.0));
            }
        }

        exits
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
