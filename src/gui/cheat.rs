use super::super::colors::*;
use bracket_lib::terminal::{to_cp437, BTerm, VirtualKeyCode};
pub enum CheatMenuResult {
    Cancel,
    NoResponse,
    TeleportToExit,
}

pub fn show_cheat_menu(ctx: &mut BTerm) -> CheatMenuResult {
    let count = 2;
    let y = 25 - count / 2;
    ctx.draw_box(15, y - 2, 31, (count + 3) as i32, c(WHITE), c(BLACK));
    ctx.print_color(18, y - 2, c(YELLOW1), c(BLACK), "Cheating!");
    ctx.print_color(18, y + count + 1, c(YELLOW1), c(BLACK), "ESCAPE to cancel");

    ctx.set(17, y, c(WHITE), c(BLACK), to_cp437('('));
    ctx.set(18, y, c(YELLOW1), c(BLACK), to_cp437('T'));
    ctx.set(19, y, c(WHITE), c(BLACK), to_cp437(')'));

    ctx.print(21, y, "Teleport to exit");

    match ctx.key {
        None => CheatMenuResult::NoResponse,
        Some(key) => match key {
            VirtualKeyCode::T => CheatMenuResult::TeleportToExit,
            VirtualKeyCode::Escape => CheatMenuResult::Cancel,
            _ => CheatMenuResult::NoResponse,
        },
    }
}
