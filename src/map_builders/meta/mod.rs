use bracket_lib::{
    random::RandomNumberGenerator,
    terminal::{DistanceAlg, Point},
};

use super::{
    super::{
        map::{
            tiles::{is_tile_walkable, Surface},
            Map,
        },
        spawner,
    },
    BuilderMap, MetaMapBuilder,
};

pub mod area_starting_position;
pub mod corridor_spawner;
pub mod cull_unreachable;
pub mod distant_exit;
pub mod door_placement;
pub mod nothing;
pub mod paint;
pub mod room_based_spawner;
pub mod room_based_stairs;
pub mod room_based_starting_position;
pub mod room_corner_rounding;
pub mod room_draw;
pub mod room_exploder;
pub mod room_sorter;
pub mod rooms_corridor_dogleg;
pub mod rooms_corridors_bsp;
pub mod rooms_corridors_lines;
pub mod rooms_corridors_nearest;
pub mod voronoi_spawner;

pub fn random_valid_points_finder(x: &X, y: &Y, data: &BuilderMap) -> usize {
    let seed_x = match x {
        X::Left => 1,
        X::Center => data.map.width / 2,
        X::Right => data.map.width - 2,
    };
    let seed_y = match y {
        Y::Top => 1,
        Y::Center => data.map.height / 2,
        Y::Bottom => data.map.height - 2,
    };

    let mut available_floors: Vec<(usize, f32)> = Vec::new();
    for (i, tile) in data.map.tiles.iter().enumerate() {
        if !is_tile_walkable(tile.surface) {
            continue;
        }
        available_floors.push((
            i,
            DistanceAlg::PythagorasSquared.distance2d(
                Point::new(i as i32 % data.map.width, i as i32 / data.map.width),
                Point::new(seed_x, seed_y),
            ),
        ));
    }

    if available_floors.is_empty() {
        panic!("No valid floors to land on");
    }

    available_floors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    available_floors[0].0
}

#[allow(dead_code)]
pub enum X {
    Left,
    Center,
    Right,
}

#[allow(dead_code)]
pub enum Y {
    Top,
    Center,
    Bottom,
}

pub fn random_position() -> (X, Y) {
    let mut rng = RandomNumberGenerator::new();
    (
        match rng.range(0, 3) {
            0 => X::Left,
            1 => X::Center,
            _ => X::Right,
        },
        match rng.range(0, 3) {
            0 => Y::Top,
            1 => Y::Center,
            _ => Y::Bottom,
        },
    )
}
