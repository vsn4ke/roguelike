use std::cmp::Ordering;

use super::super::{
    colors::*,
    item::{Consumable, Equipped, InBackpack},
    logger::log_display,
    unit::{Attribute, Attributes, Pools},
    Entity, Map, Name,
};
use bracket_lib::terminal::{to_cp437, BTerm, TextBlock, BACKEND_INTERNAL, RGB};
use specs::prelude::*;

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

fn draw_attribute(name: &str, attribute: &Attribute, y: i32, ctx: &mut BTerm) {
    let bg = c(BLACK);
    let fg = c(GRAY5);

    ctx.print_color(50, y, fg, bg, name);
    let color: RGB = match attribute.modifiers.cmp(&0) {
        Ordering::Greater => c(RED1),
        Ordering::Equal => c(WHITE),
        Ordering::Less => c(GREEN5),
    };

    ctx.print_color(
        67,
        y,
        color,
        bg,
        &format!("{}", attribute.base + attribute.modifiers),
    );
    ctx.print_color(73, y, color, bg, &format!("{}", attribute.bonus()));
    if attribute.bonus() > 0 {
        ctx.set(72, y, color, bg, to_cp437('+'));
    }
}

pub fn draw_ui(ecs: &World, ctx: &mut BTerm) {
    let fg = c(GREY);
    let bg = c(BLACK);

    //Layout
    draw_hollow_box(ctx, 0, 0, 79, 59, fg, bg);
    draw_hollow_box(ctx, 0, 0, 49, 45, fg, bg);
    draw_hollow_box(ctx, 0, 45, 79, 14, fg, bg);
    draw_hollow_box(ctx, 49, 0, 30, 8, fg, bg);
    ctx.set(0, 45, fg, bg, to_cp437('├'));
    ctx.set(49, 8, fg, bg, to_cp437('├'));
    ctx.set(49, 0, fg, bg, to_cp437('┬'));
    ctx.set(49, 45, fg, bg, to_cp437('┴'));
    ctx.set(79, 8, fg, bg, to_cp437('┤'));
    ctx.set(79, 45, fg, bg, to_cp437('┤'));

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
    let health = format!(
        " HP: {} / {} ",
        player_pools.hit_points.current, player_pools.hit_points.max
    );
    let mana = format!(
        " Mana: {} / {} ",
        player_pools.mana.current, player_pools.mana.max
    );

    ctx.print_color(50, 1, c(WHITE), bg, &health);
    ctx.print_color(50, 2, c(WHITE), bg, &mana);

    ctx.draw_bar_horizontal(
        64,
        1,
        14,
        player_pools.hit_points.current,
        player_pools.hit_points.max,
        c(RED3),
        bg,
    );
    ctx.draw_bar_horizontal(
        64,
        2,
        14,
        player_pools.mana.current,
        player_pools.mana.max,
        c(BLUE3),
        bg,
    );

    let attributes = ecs.read_storage::<Attributes>();
    let attribute = attributes.get(*player_entity).unwrap();

    draw_attribute("Might: ", &attribute.might, 4, ctx);
    draw_attribute("Quickness: ", &attribute.quickness, 5, ctx);
    draw_attribute("Fitness: ", &attribute.fitness, 6, ctx);
    draw_attribute("Intelligence: ", &attribute.intelligence, 7, ctx);

    // Initiative and weight
    ctx.print_color(
        50,
        9,
        c(WHITE),
        bg,
        &format!(
            "{:.0} kg ({:.0} kg max)",
            attribute.total_weight,
            (attribute.might.base + attribute.might.modifiers) as f32 * 1.5
        ),
    );
    ctx.print_color(
        50,
        10,
        c(WHITE),
        bg,
        &format!(
            "Initiative Penalty: {:.0}",
            attribute.total_initiative_penalty
        ),
    );

    //Right
    // Equipped
    let mut y = 14;
    let equipped = ecs.read_storage::<Equipped>();
    let name = ecs.read_storage::<Name>();
    for (equipped_by, item_name) in (&equipped, &name).join() {
        if equipped_by.owner == *player_entity {
            ctx.print_color(50, y, c(WHITE), bg, &item_name.name);
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
            ctx.print_color(50, y, c(YELLOW1), bg, &format!("↑{}", index));
            ctx.print_color(53, y, c(GREEN5), bg, &item_name.name);
            y += 1;
            index += 1;
        }
    }
    //Bottom
    let mut block = TextBlock::new(1, 46, 79, 58);
    block.print(&log_display()).unwrap();
    block.render(&mut BACKEND_INTERNAL.lock().consoles[0].console);
}
