use super::super::{camera::get_screen_bounds, colors::*, unit::Viewshed, State};
use super::menu::ItemMenuResult;
use bracket_lib::terminal::{BTerm, DistanceAlg, Point};
use specs::prelude::*;

pub fn ranged_target(
    gs: &mut State,
    ctx: &mut BTerm,
    range: i32,
) -> (ItemMenuResult, Option<Point>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    let player_pos = gs.ecs.fetch::<Point>();
    let viewshed = gs.ecs.read_storage::<Viewshed>();

    let (min_x, max_x, min_y, max_y) = get_screen_bounds(*player_pos);

    ctx.print_color(5, 0, c(YELLOW1), c(BLACK), "Select Target:");

    // Highlight cells
    let mut available_cells = Vec::new();
    let visible = viewshed.get(*player_entity);
    if let Some(visible) = visible {
        for idx in visible.visible_tiles.iter() {
            let distance = DistanceAlg::PythagorasSquared.distance2d(*player_pos, *idx);
            if distance <= (range * range) as f32 {
                let screen_x = idx.x - min_x;
                let screen_y = idx.y - min_y;
                if screen_x > 1
                    && screen_x < (max_x - min_x) - 1
                    && screen_y > 1
                    && screen_y < (max_y - min_y) - 1
                {
                    ctx.set_bg(screen_x, screen_y, c(BLUE1));
                    available_cells.push(idx)
                }
            }
        }
    } else {
        return (ItemMenuResult::Cancel, None);
    }

    //Draw mouse cursor
    let mouse_pt = ctx.mouse_point();
    let mut valid_target = false;
    let mut mouse_pos = mouse_pt;
    mouse_pos.x += min_x;
    mouse_pos.y += min_y;

    for idx in available_cells.iter() {
        if idx.x == mouse_pos.x && idx.y == mouse_pos.y {
            valid_target = true;
        }
    }
    if valid_target {
        ctx.set_bg(mouse_pt.x, mouse_pt.y, c(BLUE5));
        if ctx.left_click {
            return (ItemMenuResult::Selected, Some(mouse_pos));
        }
    } else {
        ctx.set_bg(mouse_pt.x, mouse_pt.y, c(RED1));
        if ctx.left_click {
            return (ItemMenuResult::Cancel, None);
        }
    }

    (ItemMenuResult::NoResponse, None)
}
