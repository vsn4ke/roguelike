use bracket_lib::terminal::{letter_to_option, to_cp437, BTerm, VirtualKeyCode};
use bracket_terminal::FontCharType;
use specs::prelude::*;

use super::{
    super::{
        colors::*,
        item::{Equipped, InBackpack, Item},
        raws::items::get_vendor_items,
        state::State,
        unit::{Vendor, VendorMode},
        Name,
    },
    menu::ItemMenuResult,
};

pub enum ItemMenuType {
    Use,
    Drop,
}

const MENU_X: usize = 12;
const MENU_WIDTH: usize = 50;

pub fn draw_menu(ctx: &mut BTerm, count: usize, menu_text: &str, y: usize) {
    ctx.draw_box(
        MENU_X,
        y - 2,
        MENU_WIDTH,
        (count + 3) as i32,
        c(WHITE),
        c(BLACK),
    );
    ctx.print_color(MENU_X + 3, y - 2, c(YELLOW1), c(BLACK), menu_text);
    ctx.print_color(
        MENU_X + 3,
        y + count + 1,
        c(YELLOW1),
        c(BLACK),
        "ESCAPE to cancel",
    );
}

pub fn draw_menu_item(ctx: &mut BTerm, j: usize, y: usize, item_name: &str) {
    ctx.set(MENU_X + 2, y, c(WHITE), c(BLACK), to_cp437('('));
    ctx.set(MENU_X + 3, y, c(YELLOW1), c(BLACK), 97 + j as FontCharType);
    ctx.set(MENU_X + 4, y, c(WHITE), c(BLACK), to_cp437(')'));
    ctx.print(MENU_X + 6, y, item_name);
}

fn draw_price_item(ctx: &mut BTerm, value: i32, y: usize) {
    let text = format!("{} silvers", value);
    let text_x = MENU_WIDTH - text.len() + 3;
    ctx.print(text_x, y, &text);
}

pub fn show_item_menu(
    gs: &mut State,
    ctx: &mut BTerm,
    item_menu_type: ItemMenuType,
) -> (ItemMenuResult, Option<Entity>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    let names = gs.ecs.read_storage::<Name>();
    let backpack = gs.ecs.read_storage::<InBackpack>();

    let count = (&backpack, &names)
        .join()
        .filter(|item| item.0.owner == *player_entity)
        .count();

    let mut y = 25 - (count / 2);

    let text = match item_menu_type {
        ItemMenuType::Use => "Inventory",
        ItemMenuType::Drop => "Drop Which Item?",
    };

    draw_menu(ctx, count, text, y);

    let mut equippable = Vec::<Entity>::new();
    for (j, (entity, _, name)) in (&gs.ecs.entities(), &backpack, &names)
        .join()
        .filter(|item| item.1.owner == *player_entity)
        .enumerate()
    {
        draw_menu_item(ctx, j, y, &name.name);
        equippable.push(entity);
        y += 1;
    }

    match_key(ctx.key, equippable, count)
}

pub fn remove_item_menu(gs: &mut State, ctx: &mut BTerm) -> (ItemMenuResult, Option<Entity>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    let names = gs.ecs.read_storage::<Name>();
    let backpack = gs.ecs.read_storage::<Equipped>();

    let count = (&backpack, &names)
        .join()
        .filter(|item| item.0.owner == *player_entity)
        .count();

    let mut y = 25 - (count / 2);
    draw_menu(ctx, count, "Remove Which Item?", y);

    let mut equippable = Vec::<Entity>::new();
    for (j, (entity, _, name)) in (&gs.ecs.entities(), &backpack, &names)
        .join()
        .filter(|item| item.1.owner == *player_entity)
        .enumerate()
    {
        draw_menu_item(ctx, j, y, &name.name);
        equippable.push(entity);
        y += 1;
    }

    match_key(ctx.key, equippable, count)
}

fn match_key(
    key: Option<VirtualKeyCode>,
    equippable: Vec<Entity>,
    count: usize,
) -> (ItemMenuResult, Option<Entity>) {
    if let Some(key) = key {
        if key == VirtualKeyCode::Escape {
            return (ItemMenuResult::Cancel, None);
        }
        let selection = letter_to_option(key) as usize;
        if selection < count {
            return (ItemMenuResult::Selected, Some(equippable[selection]));
        }
    }
    (ItemMenuResult::NoResponse, None)
}

#[derive(PartialEq, Copy, Clone)]
pub enum VendorResult {
    NoResponse,
    Cancel,
    Sell,
    BuyMode,
    SellMode,
    Buy,
}

fn vendor_sell_menu(
    gs: &mut State,
    ctx: &mut BTerm,
) -> (VendorResult, Option<Entity>, Option<String>, Option<i32>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    let names = gs.ecs.read_storage::<Name>();
    let backpack = gs.ecs.read_storage::<InBackpack>();
    let items = gs.ecs.read_storage::<Item>();
    let entities = gs.ecs.entities();

    let inventory = (&backpack, &names)
        .join()
        .filter(|item| item.0.owner == *player_entity);
    let count = inventory.count();

    let mut y = 25 - (count / 2);
    draw_menu(
        ctx,
        count,
        "Sell Which Item? (space to switch to buy mode)",
        y,
    );

    let mut equippable = Vec::<Entity>::new();
    for (j, (entity, _, name, item)) in (&entities, &backpack, &names, &items)
        .join()
        .filter(|item| item.1.owner == *player_entity)
        .enumerate()
    {
        draw_menu_item(ctx, j, y, &name.name);
        draw_price_item(ctx, item.base_value, y);
        equippable.push(entity);
        y += 1;
    }

    match ctx.key {
        None => (VendorResult::NoResponse, None, None, None),
        Some(key) => match key {
            VirtualKeyCode::Space => (VendorResult::BuyMode, None, None, None),
            VirtualKeyCode::Escape => (VendorResult::Cancel, None, None, None),
            _ => {
                let selection = letter_to_option(key);
                if selection > -1 && selection < count as i32 {
                    return (
                        VendorResult::Sell,
                        Some(equippable[selection as usize]),
                        None,
                        None,
                    );
                }
                (VendorResult::NoResponse, None, None, None)
            }
        },
    }
}

fn vendor_buy_menu(
    gs: &mut State,
    ctx: &mut BTerm,
    vendor: Entity,
) -> (VendorResult, Option<Entity>, Option<String>, Option<i32>) {
    use crate::raws::*;

    let vendors = gs.ecs.read_storage::<Vendor>();

    let inventory = get_vendor_items(
        &vendors.get(vendor).unwrap().categories,
        &RAWS.lock().unwrap(),
    );
    let count = inventory.len();

    let mut y = 25 - count / 2;
    draw_menu(
        ctx,
        count,
        "Buy Which Item? (space to switch to sell mode)",
        y,
    );

    for (j, sale) in inventory.iter().enumerate() {
        draw_menu_item(ctx, j, y, &sale.0);
        draw_price_item(ctx, sale.1, y);
        y += 1;
    }

    match ctx.key {
        None => (VendorResult::NoResponse, None, None, None),
        Some(key) => match key {
            VirtualKeyCode::Space => (VendorResult::SellMode, None, None, None),
            VirtualKeyCode::Escape => (VendorResult::Cancel, None, None, None),
            _ => {
                let selection = letter_to_option(key);
                if selection > -1 && selection < count as i32 {
                    return (
                        VendorResult::Buy,
                        None,
                        Some(inventory[selection as usize].0.clone()),
                        Some(inventory[selection as usize].1),
                    );
                }
                (VendorResult::NoResponse, None, None, None)
            }
        },
    }
}

pub fn show_vendor_menu(
    gs: &mut State,
    ctx: &mut BTerm,
    vendor: Entity,
    mode: VendorMode,
) -> (VendorResult, Option<Entity>, Option<String>, Option<i32>) {
    match mode {
        VendorMode::Buy => vendor_buy_menu(gs, ctx, vendor),
        VendorMode::Sell => vendor_sell_menu(gs, ctx),
    }
}
