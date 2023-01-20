use std::cmp::Ordering;

use super::{
    super::{
        colors::*,
        item::{Consumable, Equipped, InBackpack},
        logger::log_display,
        unit::{Attribute, Attributes, Pools},
        Entity, Map, Name, CONSOLE_HEIGHT, CONSOLE_WIDTH,
    },
    draw_bar_horizontal, draw_hollow_box,
};
use bracket_lib::terminal::{to_cp437, BTerm, TextBlock, BACKEND_INTERNAL, RGB};
use specs::prelude::*;

const C_WIDTH: i32 = CONSOLE_WIDTH as i32 - 1;
const C_HEIGHT: i32 = CONSOLE_HEIGHT as i32 - 1;
const ATTR_BOX_HEIGHT: i32 = 9;
const ATTR_BOX_WIDTH: i32 = 31;
const LOG_BOX_HEIGHT: i32 = 16;
const V_BAR_Y: i32 = C_WIDTH - ATTR_BOX_WIDTH;
const H_BAR_2_X: i32 = C_HEIGHT - LOG_BOX_HEIGHT;
const LOOT_BOX_HEIGHT: i32 = H_BAR_2_X - ATTR_BOX_HEIGHT;

fn draw_attribute(name: &str, attribute: &Attribute, y: i32, ctx: &mut BTerm) {
    let bg = c(BLACK);
    let fg = c(GREY);

    ctx.print_color(V_BAR_Y + 2, y, fg, bg, name);
    let color: RGB = match attribute.modifiers.cmp(&0) {
        Ordering::Greater => c(RED1),
        Ordering::Equal => c(WHITE),
        Ordering::Less => c(GREEN5),
    };

    ctx.print_color(
        V_BAR_Y + 17,
        y,
        color,
        bg,
        &format!("{}", attribute.base + attribute.modifiers),
    );
    ctx.print_color(
        V_BAR_Y + 25,
        y,
        color,
        bg,
        &format!("{}", attribute.bonus()),
    );
    if attribute.bonus() > 0 {
        ctx.set(V_BAR_Y + 24, y, color, bg, to_cp437('+'));
    }
    if attribute.bonus() < 0 {
        ctx.set(V_BAR_Y + 24, y, color, bg, to_cp437('-'));
    }
}

pub fn draw_ui(ecs: &World, ctx: &mut BTerm) {
    let fg = c(GREY);
    let bg = c(BLACK);

    //Layout
    draw_hollow_box(ctx, 0, 0, C_WIDTH, C_HEIGHT, fg, bg); //big box
    draw_hollow_box(ctx, 0, H_BAR_2_X, C_WIDTH, LOG_BOX_HEIGHT, fg, bg);
    draw_hollow_box(ctx, V_BAR_Y, 0, ATTR_BOX_WIDTH, ATTR_BOX_HEIGHT, fg, bg);
    draw_hollow_box(
        ctx,
        V_BAR_Y,
        ATTR_BOX_HEIGHT,
        ATTR_BOX_WIDTH,
        LOOT_BOX_HEIGHT,
        fg,
        bg,
    );

    ctx.set(0, H_BAR_2_X, fg, bg, to_cp437('├'));
    ctx.set(V_BAR_Y, ATTR_BOX_HEIGHT, fg, bg, to_cp437('├'));
    ctx.set(V_BAR_Y, 0, fg, bg, to_cp437('┬'));
    ctx.set(V_BAR_Y, H_BAR_2_X, fg, bg, to_cp437('┴'));
    ctx.set(C_WIDTH, ATTR_BOX_HEIGHT, fg, bg, to_cp437('┤'));
    ctx.set(C_WIDTH, H_BAR_2_X, fg, bg, to_cp437('┤'));

    //map name
    let map = ecs.fetch::<Map>();
    let name_len = map.name.len() as i32;
    let x = 23 - name_len / 2;
    ctx.set(x, 0, fg, bg, to_cp437('┤'));
    ctx.set(x + name_len + 1, 0, fg, bg, to_cp437('├'));
    ctx.print_color(x + 1, 0, c(WHITE), bg, &map.name);
    std::mem::drop(map);

    //top right
    let pools = ecs.read_storage::<Pools>();
    let player_entity = ecs.fetch::<Entity>();
    let player_pools = pools.get(*player_entity).unwrap();
    let attributes = ecs.read_storage::<Attributes>();
    let attribute = attributes.get(*player_entity).unwrap();

    let health = format!(
        "HP: {} / {} ",
        player_pools.hit_points.current, player_pools.hit_points.max
    );
    let mana = format!(
        "Mana: {} / {} ",
        player_pools.mana.current, player_pools.mana.max
    );
    let xp = format!("Level:  {} ", attribute.level);

    ctx.print_color(V_BAR_Y + 2, 1, c(WHITE), bg, &health);
    ctx.print_color(V_BAR_Y + 2, 2, c(WHITE), bg, &mana);
    ctx.print_color(V_BAR_Y + 2, 3, c(WHITE), bg, &xp);

    draw_bar_horizontal(
        ctx,
        64,
        1,
        14,
        player_pools.hit_points.current,
        player_pools.hit_points.max,
        c(RED3),
        bg,
    );
    draw_bar_horizontal(
        ctx,
        64,
        2,
        14,
        player_pools.mana.current,
        player_pools.mana.max,
        c(BLUE3),
        bg,
    );

    draw_bar_horizontal(
        ctx,
        64,
        3,
        14,
        player_pools.xp,
        attribute.level * 1000,
        c(YELLOW3),
        bg,
    );

    draw_attribute("Might: ", &attribute.might, 5, ctx);
    draw_attribute("Quickness: ", &attribute.quickness, 6, ctx);
    draw_attribute("Fitness: ", &attribute.fitness, 7, ctx);
    draw_attribute("Intelligence: ", &attribute.intelligence, 8, ctx);

    // Initiative and weight
    ctx.print_color(
        V_BAR_Y + 2,
        ATTR_BOX_HEIGHT + 1,
        c(WHITE),
        bg,
        &format!(
            "{:.1} kg ({:.1} kg max)",
            attribute.total_weight,
            attribute.max_weight()
        ),
    );
    ctx.print_color(
        V_BAR_Y + 2,
        ATTR_BOX_HEIGHT + 2,
        c(WHITE),
        bg,
        &format!(
            "Initiative Penalty: {:.0}",
            attribute.total_initiative_penalty
        ),
    );
    ctx.print_color(
        V_BAR_Y + 2,
        ATTR_BOX_HEIGHT + 4,
        c(YELLOW5),
        bg,
        &format!(
            "Gold: {} Silvers: {}",
            player_pools.money / 100,
            player_pools.money % 100
        ),
    );

    //Right
    // Equipped
    let mut y = ATTR_BOX_HEIGHT + 6;
    let equipped = ecs.read_storage::<Equipped>();
    let name = ecs.read_storage::<Name>();
    for (equipped_by, item_name) in (&equipped, &name).join() {
        if equipped_by.owner == *player_entity {
            ctx.print_color(V_BAR_Y + 2, y, c(WHITE), bg, &item_name.name);
            y += 1;
        }
    }
    // Consumables
    y += 1;
    let consumables = ecs.read_storage::<Consumable>();
    let backpack = ecs.read_storage::<InBackpack>();
    let mut index = 1;
    for (carried_by, _, item_name) in (&backpack, &consumables, &name).join() {
        if carried_by.owner == *player_entity && index < 10 {
            ctx.print_color(V_BAR_Y + 2, y, c(YELLOW1), bg, &format!("↑{}", index));
            ctx.print_color(V_BAR_Y + 5, y, c(GREEN5), bg, &item_name.name);
            y += 1;
            index += 1;
        }
    }
    //Bottom
    let mut block = TextBlock::new(1, H_BAR_2_X + 1, C_WIDTH - 2, LOG_BOX_HEIGHT);
    block.print(&log_display()).unwrap();
    block.render(&mut BACKEND_INTERNAL.lock().consoles[0].console);
}
