use super::{colors::*, map::themes::get_tile_glyph, Hidden, Map, Position, Renderable};
use bracket_lib::prelude::Algorithm2D;
use bracket_lib::terminal::{to_cp437, BTerm, Point};
use specs::prelude::*;

const SHOW_BOUNDARIES: bool = true;

pub fn render_camera(ecs: &World, ctx: &mut BTerm) {
    let positions = ecs.read_storage::<Position>();
    let renderables = ecs.read_storage::<Renderable>();
    let hidden = ecs.read_storage::<Hidden>();
    let map = ecs.fetch::<Map>();
    let player_pos = ecs.fetch::<Point>();

    let (min_x, max_x, min_y, max_y) = get_screen_bounds(*player_pos);

    for (y, tile_y) in (min_y..max_y).enumerate() {
        for (x, tile_x) in (min_x..max_x).enumerate() {
            let tile = Point::new(tile_x, tile_y);
            if map.in_bounds(tile) {
                let idx = map.point2d_to_index(tile);
                if map.tiles[idx].revealed {
                    let (glyph, fg, bg) = get_tile_glyph(idx, &map);
                    ctx.set(x, y, fg, bg, glyph);
                }
            } else if SHOW_BOUNDARIES {
                ctx.set(x, y, c(GRAY1), c(BLACK), to_cp437('·'));
            }
        }
    }

    let mut data = (&positions, &renderables, !&hidden)
        .join()
        .collect::<Vec<_>>();
    data.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));
    for (pos, render, _) in data.iter() {
        if !map.tiles[map.point2d_to_index(pos.into_point())].visible {
            continue;
        }
        ctx.set(
            pos.x - min_x,
            pos.y - min_y,
            render.fg,
            render.bg,
            render.glyph,
        );
    }
}

pub fn get_screen_bounds(pos: Point) -> (i32, i32, i32, i32) {
    let (x_chars, y_chars) = (48, 44);

    let min_x = pos.x - x_chars / 2;
    let max_x = min_x + x_chars;
    let min_y = pos.y - y_chars / 2;
    let max_y = min_y + y_chars;

    (min_x, max_x, min_y, max_y)
}

pub fn render_debug_map(map: &Map, ctx: &mut BTerm) {
    let player_pos = Point::new(map.width / 2, map.height / 2);
    let (min_x, max_x, min_y, max_y) = get_screen_bounds(player_pos);

    for (y, tile_y) in (min_y..max_y).enumerate() {
        for (x, tile_x) in (min_x..max_x).enumerate() {
            if tile_x > 0 && tile_x < map.width - 1 && tile_y > 0 && tile_y < map.height - 1 {
                let idx = map.coord_to_index(tile_x, tile_y);
                if map.tiles[idx].revealed {
                    let (glyph, fg, bg) = get_tile_glyph(idx, map);
                    ctx.set(x, y, fg, bg, glyph);
                }
            } else if SHOW_BOUNDARIES {
                ctx.set(x, y, c(GRAY1), c(BLACK), to_cp437('·'));
            }
        }
    }
}
