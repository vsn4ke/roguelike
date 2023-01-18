use bracket_lib::terminal::{letter_to_option, to_cp437, BTerm, VirtualKeyCode};
use bracket_terminal::FontCharType;
use specs::prelude::*;

use super::super::{
    colors::*,
    item::{Equipped, InBackpack},
    state::State,
    Name,
};
use super::menu::ItemMenuResult;

pub fn show_inventory(gs: &mut State, ctx: &mut BTerm) -> (ItemMenuResult, Option<Entity>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    let names = gs.ecs.read_storage::<Name>();
    let backpack = gs.ecs.read_storage::<InBackpack>();
    let entities = gs.ecs.entities();

    let inventory = (&backpack, &names)
        .join()
        .filter(|item| item.0.owner == *player_entity);
    let count = inventory.count();

    let mut y = (25 - (count / 2)) as i32;

    ctx.draw_box(15, y - 2, 31, (count + 3) as i32, c(WHITE), c(BLACK));
    ctx.print_color(18, y - 2, c(YELLOW1), c(BLACK), "Inventory");
    ctx.print_color(
        18,
        y + count as i32 + 1,
        c(YELLOW1),
        c(BLACK),
        "ESCAPE to cancel",
    );

    let mut equippable: Vec<Entity> = Vec::new();

    for (j, (entity, _pack, name)) in (&entities, &backpack, &names)
        .join()
        .filter(|item| item.1.owner == *player_entity)
        .enumerate()
    {
        ctx.set(17, y, c(WHITE), c(BLACK), to_cp437('('));
        ctx.set(18, y, c(YELLOW1), c(BLACK), 97 + j as FontCharType);
        ctx.set(19, y, c(WHITE), c(BLACK), to_cp437(')'));

        ctx.print(21, y, &name.name.to_string());
        equippable.push(entity);
        y += 1;
    }

    match ctx.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => match key {
            VirtualKeyCode::Escape => (ItemMenuResult::Cancel, None),
            _ => {
                let selection = letter_to_option(key);
                if selection > -1 && selection < count as i32 {
                    return (
                        ItemMenuResult::Selected,
                        Some(equippable[selection as usize]),
                    );
                }
                (ItemMenuResult::NoResponse, None)
            }
        },
    }
}

pub fn drop_item_menu(gs: &mut State, ctx: &mut BTerm) -> (ItemMenuResult, Option<Entity>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    let names = gs.ecs.read_storage::<Name>();
    let backpack = gs.ecs.read_storage::<InBackpack>();
    let entities = gs.ecs.entities();

    let inventory = (&backpack, &names)
        .join()
        .filter(|item| item.0.owner == *player_entity);
    let count = inventory.count();

    let mut y = (25 - (count / 2)) as i32;
    ctx.draw_box(15, y - 2, 31, (count + 3) as i32, c(WHITE), c(BLACK));
    ctx.print_color(18, y - 2, c(YELLOW1), c(BLACK), "Drop Which Item?");
    ctx.print_color(
        18,
        y + count as i32 + 1,
        c(YELLOW1),
        c(BLACK),
        "ESCAPE to cancel",
    );

    let mut equippable: Vec<Entity> = Vec::new();

    for (j, (entity, _pack, name)) in (&entities, &backpack, &names)
        .join()
        .filter(|item| item.1.owner == *player_entity)
        .enumerate()
    {
        ctx.set(17, y, c(WHITE), c(BLACK), to_cp437('('));
        ctx.set(18, y, c(YELLOW1), c(BLACK), 97 + j as FontCharType);
        ctx.set(19, y, c(WHITE), c(BLACK), to_cp437(')'));

        ctx.print(21, y, &name.name.to_string());
        equippable.push(entity);
        y += 1;
    }

    match ctx.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => match key {
            VirtualKeyCode::Escape => (ItemMenuResult::Cancel, None),
            _ => {
                let selection = letter_to_option(key);
                if selection > -1 && selection < count as i32 {
                    return (
                        ItemMenuResult::Selected,
                        Some(equippable[selection as usize]),
                    );
                }
                (ItemMenuResult::NoResponse, None)
            }
        },
    }
}

pub fn remove_item_menu(gs: &mut State, ctx: &mut BTerm) -> (ItemMenuResult, Option<Entity>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    let names = gs.ecs.read_storage::<Name>();
    let backpack = gs.ecs.read_storage::<Equipped>();
    let entities = gs.ecs.entities();

    let inventory = (&backpack, &names)
        .join()
        .filter(|item| item.0.owner == *player_entity);
    let count = inventory.count();

    let mut y = (25 - (count / 2)) as i32;
    ctx.draw_box(15, y - 2, 31, (count + 3) as i32, c(WHITE), c(BLACK));
    ctx.print_color(18, y - 2, c(YELLOW1), c(BLACK), "Remove Which Item?");
    ctx.print_color(
        18,
        y + count as i32 + 1,
        c(YELLOW1),
        c(BLACK),
        "ESCAPE to cancel",
    );

    let mut equippable: Vec<Entity> = Vec::new();

    for (j, (entity, _pack, name)) in (&entities, &backpack, &names)
        .join()
        .filter(|item| item.1.owner == *player_entity)
        .enumerate()
    {
        ctx.set(17, y, c(WHITE), c(BLACK), to_cp437('('));
        ctx.set(18, y, c(YELLOW1), c(BLACK), 97 + j as FontCharType);
        ctx.set(19, y, c(WHITE), c(BLACK), to_cp437(')'));

        ctx.print(21, y, &name.name.to_string());
        equippable.push(entity);
        y += 1;
    }

    match ctx.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => match key {
            VirtualKeyCode::Escape => (ItemMenuResult::Cancel, None),
            _ => {
                let selection = letter_to_option(key);
                if selection > -1 && selection < count as i32 {
                    return (
                        ItemMenuResult::Selected,
                        Some(equippable[selection as usize]),
                    );
                }
                (ItemMenuResult::NoResponse, None)
            }
        },
    }
}
