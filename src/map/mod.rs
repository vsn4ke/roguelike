pub mod master;
pub mod spatial;
pub mod themes;
pub mod tiles;

use tiles::{is_tile_opaque, is_tile_walkable, Surface};

use super::{
    colors::{c, BLACK},
    OtherLevelPosition, Position,
};

use bracket_lib::{
    prelude::{Algorithm2D, BaseMap, SmallVec},
    terminal::{DistanceAlg, Point, Rect, RGB},
};

#[derive(Clone)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub tiles: Vec<Tile>,
    pub rooms: Vec<Rect>,
    pub name: String,
    pub outdoors: bool,
}

#[derive(Clone, Default)]
pub struct Tile {
    pub surface: Surface,
    pub revealed: bool,
    pub visible: bool,
    pub block_visibility: bool,
    pub bloodstains: bool,
    pub light: RGB,
}

impl Tile {
    pub fn new() -> Self {
        Self {
            surface: Surface::Wall,
            revealed: false,
            visible: false,
            block_visibility: false,
            bloodstains: false,
            light: c(BLACK),
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
            outdoors: true,
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
