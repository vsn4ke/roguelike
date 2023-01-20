use bracket_lib::terminal::{to_cp437, BTerm, RGB};

pub mod cheat;
pub mod inventory;
pub mod menu;
pub mod target;
pub mod tooltips;
pub mod ui;

#[allow(clippy::too_many_arguments)]
pub fn draw_bar_horizontal(
    console: &mut BTerm,
    sx: i32,
    sy: i32,
    width: i32,
    n: i32,
    max: i32,
    fg: RGB,
    bg: RGB,
) {
    let percent = n as f32 / max as f32;
    let fill_width = (percent * width as f32) as i32;
    for x in 0..width {
        if x < fill_width {
            console.set(sx + x, sy, fg, bg, to_cp437('█'));
        } else {
            console.set(sx + x, sy, fg, bg, to_cp437('.'));
        }
    }
}

pub fn draw_hollow_box(
    console: &mut BTerm,
    sx: i32,
    sy: i32,
    width: i32,
    height: i32,
    fg: RGB,
    bg: RGB,
) {
    console.set(sx, sy, fg, bg, to_cp437('┌'));
    console.set(sx + width, sy, fg, bg, to_cp437('┐'));
    console.set(sx, sy + height, fg, bg, to_cp437('└'));
    console.set(sx + width, sy + height, fg, bg, to_cp437('┘'));
    for x in sx + 1..sx + width {
        console.set(x, sy, fg, bg, to_cp437('─'));
        console.set(x, sy + height, fg, bg, to_cp437('─'));
    }
    for y in sy + 1..sy + height {
        console.set(sx, y, fg, bg, to_cp437('│'));
        console.set(sx + width, y, fg, bg, to_cp437('│'));
    }
}
