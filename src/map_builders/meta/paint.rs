use bracket_lib::{prelude::Algorithm2D, terminal::Point};

use super::{Map, Surface};
use std::cmp::{max, min};

#[allow(dead_code)]
#[derive(PartialEq, Copy, Clone)]
pub enum Symmetry {
    None,
    Horizontal,
    Vertical,
    Both,
}

#[allow(dead_code)]
pub fn paint(map: &mut Map, mode: Symmetry, brush_size: i32, p: Point) {
    match mode {
        Symmetry::None => apply_paint(map, brush_size, p),
        Symmetry::Horizontal => {
            let center_x = map.width / 2;
            if p.x == center_x {
                apply_paint(map, brush_size, p);
            } else {
                let dist_x = i32::abs(center_x - p.x);
                apply_paint(map, brush_size, Point::new(center_x + dist_x, p.y));
                apply_paint(map, brush_size, Point::new(center_x - dist_x, p.y));
            }
        }
        Symmetry::Vertical => {
            let center_y = map.height / 2;
            if p.y == center_y {
                apply_paint(map, brush_size, p);
            } else {
                let dist_y = i32::abs(center_y - p.y);
                apply_paint(map, brush_size, Point::new(p.x, center_y + dist_y));
                apply_paint(map, brush_size, Point::new(p.x, center_y - dist_y));
            }
        }
        Symmetry::Both => {
            let center_x = map.width / 2;
            let center_y = map.height / 2;
            if p.x == center_x && p.y == center_y {
                apply_paint(map, brush_size, p);
            } else {
                let dist_x = i32::abs(center_x - p.x);
                apply_paint(map, brush_size, Point::new(center_x + dist_x, p.y));
                apply_paint(map, brush_size, Point::new(center_x - dist_x, p.y));
                let dist_y = i32::abs(center_y - p.y);
                apply_paint(map, brush_size, Point::new(p.x, center_y + dist_y));
                apply_paint(map, brush_size, Point::new(p.x, center_y - dist_y));
            }
        }
    }
}

#[allow(dead_code)]
pub fn apply_paint(map: &mut Map, brush_size: i32, p: Point) {
    match brush_size {
        1 => {
            let idx = map.point2d_to_index(p);
            map.tiles[idx].surface = Surface::Floor;
        }

        _ => {
            let half_brush_size = brush_size / 2;
            for brush_y in p.y - half_brush_size..p.y + half_brush_size {
                for brush_x in p.x - half_brush_size..p.x + half_brush_size {
                    if brush_x > 1
                        && brush_x < map.width - 1
                        && brush_y > 1
                        && brush_y < map.height - 1
                    {
                        let idx = map.coord_to_index(brush_x, brush_y);
                        map.tiles[idx].surface = Surface::Floor;
                    }
                }
            }
        }
    }
}

pub fn apply_horizontal_tunnel(map: &mut Map, x1: i32, x2: i32, y: i32) -> Vec<usize> {
    let mut corridor = Vec::new();
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = map.coord_to_index(x, y);
        if idx > 0 && idx < map.width as usize * map.height as usize {
            map.tiles[idx].surface = Surface::Floor;
            corridor.push(idx);
        }
    }
    corridor
}

pub fn apply_vertical_tunnel(map: &mut Map, y1: i32, y2: i32, x: i32) -> Vec<usize> {
    let mut corridor = Vec::new();
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = map.coord_to_index(x, y);
        if idx > 0 && idx < map.width as usize * map.height as usize {
            map.tiles[idx].surface = Surface::Floor;
            corridor.push(idx);
        }
    }
    corridor
}

pub fn draw_corridor(map: &mut Map, x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<usize> {
    let mut corridor = Vec::new();
    let mut x = x1;
    let mut y = y1;

    while x != x2 || y != y2 {
        if x < x2 {
            x += 1;
        } else if x > x2 {
            x -= 1;
        } else if y < y2 {
            y += 1;
        } else if y > y2 {
            y -= 1;
        }

        let idx = map.coord_to_index(x, y);

        if map.tiles[idx].surface != Surface::Floor {
            map.tiles[idx].surface = Surface::Floor;
            corridor.push(idx);
        }
    }
    corridor
}
