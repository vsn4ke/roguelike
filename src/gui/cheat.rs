use super::inventory::{draw_menu, draw_menu_item};
use bracket_lib::terminal::{BTerm, VirtualKeyCode};
pub enum CheatMenuResult {
    Cancel,
    NoResponse,
    TeleportToExit,
    Heal,
    RevealMap,
    GodMode,
}

pub fn show_cheat_menu(ctx: &mut BTerm) -> CheatMenuResult {
    let menu = [
        ('t', "Teleport to exit"),
        ('h', "Heal all wounds"),
        ('r', "Reveal the map"),
        ('g', "God Mode"),
    ];

    let count = menu.len();
    let y = 25 - count / 2;

    draw_menu(ctx, count, "Cheating!", y);
    for (i, (char, text)) in menu.iter().enumerate() {
        draw_menu_item(ctx, *char as usize - 97, y + i, text);
    }

    match ctx.key {
        None => CheatMenuResult::NoResponse,
        Some(key) => match key {
            VirtualKeyCode::T => CheatMenuResult::TeleportToExit,
            VirtualKeyCode::H => CheatMenuResult::Heal,
            VirtualKeyCode::R => CheatMenuResult::RevealMap,
            VirtualKeyCode::G => CheatMenuResult::GodMode,
            VirtualKeyCode::Escape => CheatMenuResult::Cancel,
            _ => CheatMenuResult::NoResponse,
        },
    }
}
