use super::{
    action::{WantsToMelee, WantsToPickupItem, WantsToUseItem},
    effect::Ranged,
    gui::menu::MainMenuSelection,
    item::{Consumable, InBackpack, Item},
    map::tiles::Surface,
    props::Door,
    raws::{
        factions::{faction_reaction, Reaction},
        RAWS,
    },
    state::{RunState, State},
    unit::{Attributes, EntityMoved, Faction, Player, Vendor, VendorMode, Viewshed},
    BlocksVisibility, Log, Map, Position, Renderable,
};
use bracket_lib::{
    prelude::Algorithm2D,
    terminal::{to_cp437, BTerm, Point, VirtualKeyCode},
};
use specs::prelude::*;

pub fn try_move_player(dx: i32, dy: i32, ecs: &mut World) -> RunState {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let mut entity_moved = ecs.write_storage::<EntityMoved>();
    let combat_stats = ecs.read_storage::<Attributes>();
    let mut map = ecs.fetch_mut::<Map>();
    let entities = ecs.entities();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();
    let factions = ecs.read_storage::<Faction>();
    let mut doors = ecs.write_storage::<Door>();
    let mut blocks_visibility = ecs.write_storage::<BlocksVisibility>();
    let mut renderables = ecs.write_storage::<Renderable>();
    let vendors = ecs.read_storage::<Vendor>();

    let mut swap_entities = Vec::<(Entity, i32, i32)>::new();
    let mut result = RunState::AwaitingInput;
    for (_, pos, viewshed, entity) in
        (&mut players, &mut positions, &mut viewsheds, &entities).join()
    {
        let new = Point::new(pos.x + dx, pos.y + dy);
        if !map.in_bounds(new) {
            return result;
        }

        let dest_idx = map.point2d_to_index(new);
        for potential_target in map.tiles[dest_idx].content.clone().iter() {
            if combat_stats.get(*potential_target).is_some() {
                if let Some(faction) = factions.get(*potential_target) {
                    let reaction = faction_reaction(&faction.name, "Player", &RAWS.lock().unwrap());
                    if reaction == Reaction::Attack || reaction == Reaction::Flee {
                        wants_to_melee
                            .insert(
                                entity,
                                WantsToMelee {
                                    target: *potential_target,
                                },
                            )
                            .expect("Add target failed");
                        return RunState::Ticking;
                    }
                }
            }

            if vendors.get(*potential_target).is_some() {
                return RunState::ShowVendor {
                    vendor: *potential_target,
                    mode: VendorMode::Sell,
                };
            }

            if let Some(door) = doors.get_mut(*potential_target) {
                door.open = true;
                blocks_visibility.remove(*potential_target);

                map.blocks_movement(dest_idx, false);
                renderables.get_mut(*potential_target).unwrap().glyph = to_cp437('/');
                viewshed.dirty = true;
                result = RunState::Ticking;
            } else {
                swap_entities.push((*potential_target, pos.x, pos.y));
                pos.x = (new.x).clamp(0, map.width - 1);
                pos.y = (new.y).clamp(0, map.height - 1);
                entity_moved
                    .insert(entity, EntityMoved {})
                    .expect("Unable to insert marker");
                viewshed.dirty = true;

                let mut ppos = ecs.write_resource::<Point>();
                ppos.x = pos.x;
                ppos.y = pos.y;

                result = RunState::Ticking;
            }
        }

        if map.is_blocked(dest_idx) {
            continue;
        }

        let mut ppos = ecs.write_resource::<Point>();
        entity_moved
            .insert(entity, EntityMoved {})
            .expect("Unable to insert marker");
        viewshed.dirty = true;

        let moving_from = map.coord_to_index(pos.x, pos.y);
        pos.x = new.x.clamp(0, map.width - 1);
        pos.y = new.y.clamp(0, map.height - 1);
        let moving_to = map.coord_to_index(pos.x, pos.y);

        map.move_entity(entity, moving_from, moving_to);

        ppos.x = pos.x;
        ppos.y = pos.y;

        result = match map.tiles[dest_idx].surface {
            Surface::DownStairs => RunState::NextLevel,
            Surface::UpStairs => RunState::PreviousLevel,
            _ => RunState::Ticking,
        }
    }

    for m in swap_entities.iter() {
        if let Some(their_pos) = positions.get_mut(m.0) {
            let moving_from = map.coord_to_index(their_pos.x, their_pos.y);
            their_pos.x = m.1;
            their_pos.y = m.2;
            let moving_to = map.coord_to_index(their_pos.x, their_pos.y);

            map.move_entity(m.0, moving_from, moving_to);
        }
    }

    result
}

fn get_item(ecs: &mut World) {
    let ppos = ecs.fetch::<Point>();
    let player_entity = ecs.fetch::<Entity>();
    let items = ecs.read_storage::<Item>();
    let positions = ecs.read_storage::<Position>();

    let mut target_item: Option<Entity> = None;
    for (item_entity, _, position) in (&ecs.entities(), &items, &positions).join() {
        if position.x == ppos.x && position.y == ppos.y {
            target_item = Some(item_entity);
        }
    }

    match target_item {
        None => Log::new()
            .append("There is nothing here to pick up.")
            .build(),
        Some(item) => {
            ecs.write_storage::<WantsToPickupItem>()
                .insert(
                    *player_entity,
                    WantsToPickupItem {
                        collected_by: *player_entity,
                        item,
                    },
                )
                .expect("Unable to insert want to pickup");
        }
    }
}

pub fn input(gs: &mut State, ctx: &mut BTerm) -> RunState {
    if ctx.shift && ctx.key.is_some() {
        if let Some(key) = match ctx.key.unwrap() {
            VirtualKeyCode::Key1 => Some(1),
            VirtualKeyCode::Key2 => Some(2),
            VirtualKeyCode::Key3 => Some(3),
            VirtualKeyCode::Key4 => Some(4),
            VirtualKeyCode::Key5 => Some(5),
            VirtualKeyCode::Key6 => Some(6),
            VirtualKeyCode::Key7 => Some(7),
            VirtualKeyCode::Key8 => Some(8),
            VirtualKeyCode::Key9 => Some(9),
            _ => None,
        } {
            return use_consumable_hotkey(gs, key - 1);
        }
    }

    match ctx.key {
        None => return RunState::AwaitingInput,
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 => {
                return try_move_player(-1, 0, &mut gs.ecs)
            }
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 => {
                return try_move_player(1, 0, &mut gs.ecs)
            }
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 => {
                return try_move_player(0, -1, &mut gs.ecs)
            }
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 => {
                return try_move_player(0, 1, &mut gs.ecs)
            }
            VirtualKeyCode::Numpad9 => return try_move_player(1, -1, &mut gs.ecs),
            VirtualKeyCode::Numpad7 => return try_move_player(-1, -1, &mut gs.ecs),
            VirtualKeyCode::Numpad3 => return try_move_player(1, 1, &mut gs.ecs),
            VirtualKeyCode::Numpad1 => return try_move_player(-1, 1, &mut gs.ecs),

            //action
            VirtualKeyCode::G => get_item(&mut gs.ecs),
            VirtualKeyCode::I => return RunState::ShowInventory,
            VirtualKeyCode::D => return RunState::ShowDropItem,
            VirtualKeyCode::R => return RunState::ShowRemoveItem,
            VirtualKeyCode::Minus => return RunState::ShowCheatMenu,
            VirtualKeyCode::Escape => {
                return RunState::MainMenu {
                    menu_selection: MainMenuSelection::Continue,
                }
            }
            _ => return RunState::AwaitingInput,
        },
    }
    RunState::Ticking
}

fn use_consumable_hotkey(gs: &mut State, key: usize) -> RunState {
    let consumables = gs.ecs.read_storage::<Consumable>();
    let backpack = gs.ecs.read_storage::<InBackpack>();
    let player_entity = gs.ecs.fetch::<Entity>();
    let mut carried = Vec::new();

    for (entity, carried_by, _) in (&gs.ecs.entities(), &backpack, &consumables).join() {
        if carried_by.owner == *player_entity {
            carried.push(entity);
        }
    }

    if key >= carried.len() {
        return RunState::Ticking;
    }

    if let Some(ranged) = gs.ecs.read_storage::<Ranged>().get(carried[key]) {
        return RunState::ShowTargeting {
            range: ranged.range,
            item: carried[key],
        };
    }
    gs.ecs
        .write_storage::<WantsToUseItem>()
        .insert(
            *player_entity,
            WantsToUseItem {
                item: carried[key],
                target: None,
            },
        )
        .expect("Unable to insert intent");
    RunState::Ticking
}
