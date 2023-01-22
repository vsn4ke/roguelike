use super::*;
use bracket_lib::terminal::BTerm;
use specs::prelude::*;

pub fn sell(ecs: &mut World, entity: Entity) {
    let price = ecs.read_storage::<Item>().get(entity).unwrap().base_value;
    ecs.write_storage::<Pools>()
        .get_mut(*ecs.fetch::<Entity>())
        .unwrap()
        .money += price;
    ecs.delete_entity(entity).expect("Unable to delete");
}

pub fn buy(ecs: &mut World, value: i32, name: &str) {
    let mut pools = ecs.write_storage::<Pools>();
    let player_pools = pools.get_mut(*ecs.fetch::<Entity>()).unwrap();
    if player_pools.money >= value {
        player_pools.money -= value;
        std::mem::drop(pools);
        let player_entity = *ecs.fetch::<Entity>();
        spawn_named_item(
            &RAWS.lock().unwrap(),
            ecs,
            name,
            SpawnType::Carried { by: player_entity },
        );
    }
}

pub fn map_generation(gen: &mut MapGen, ctx: &mut BTerm) -> RunState {
    if !SHOW_MAPGEN_VISUALIZER {
        return gen.next_state.unwrap();
    }

    ctx.cls();

    if gen.index < gen.history.len() {
        render_debug_map(&gen.history[gen.index], ctx);
    }

    gen.timer += ctx.frame_time_ms;

    if gen.timer > 300.0 {
        gen.timer = 0.0;
        gen.index += 1;
        if gen.index >= gen.history.len() {
            return gen.next_state.unwrap();
        }
    }

    RunState::MapGeneration
}

pub fn show_inv_use(gs: &mut State, ctx: &mut BTerm) -> RunState {
    let (result, entity) = show_item_menu(gs, ctx, ItemMenuType::Use);
    match result {
        ItemMenuResult::Cancel => RunState::AwaitingInput,
        ItemMenuResult::NoResponse => RunState::ShowInventory,
        ItemMenuResult::Selected => {
            let item_entity = entity.unwrap();

            if let Some(is_ranged) = gs.ecs.read_storage::<Ranged>().get(item_entity) {
                return RunState::ShowTargeting {
                    range: is_ranged.range,
                    item: item_entity,
                };
            }

            gs.ecs
                .write_storage::<WantsToUseItem>()
                .insert(
                    *gs.ecs.fetch::<Entity>(),
                    WantsToUseItem {
                        item: item_entity,
                        target: None,
                    },
                )
                .expect("Unable to insert intent");
            RunState::Ticking
        }
    }
}

pub fn show_inv_drop(gs: &mut State, ctx: &mut BTerm) -> RunState {
    let (result, entity) = show_item_menu(gs, ctx, ItemMenuType::Drop);
    match result {
        ItemMenuResult::Cancel => RunState::AwaitingInput,
        ItemMenuResult::NoResponse => RunState::ShowDropItem,
        ItemMenuResult::Selected => {
            gs.ecs
                .write_storage::<WantsToDropItem>()
                .insert(
                    *gs.ecs.fetch::<Entity>(),
                    WantsToDropItem {
                        item: entity.unwrap(),
                    },
                )
                .expect("Unable to insert intent");
            RunState::Ticking
        }
    }
}

pub fn show_inv_remove(gs: &mut State, ctx: &mut BTerm) -> RunState {
    let (result, entity) = remove_item_menu(gs, ctx);
    match result {
        ItemMenuResult::Cancel => RunState::AwaitingInput,
        ItemMenuResult::NoResponse => RunState::ShowRemoveItem,
        ItemMenuResult::Selected => {
            gs.ecs
                .write_storage::<WantsToRemoveItem>()
                .insert(
                    *gs.ecs.fetch::<Entity>(),
                    WantsToRemoveItem {
                        item: entity.unwrap(),
                    },
                )
                .expect("Unable to insert intent");
            RunState::Ticking
        }
    }
}

pub fn show_targeting(gs: &mut State, ctx: &mut BTerm, range: i32, item: Entity) -> RunState {
    let (result, point) = ranged_target(gs, ctx, range);
    match result {
        ItemMenuResult::Cancel => RunState::AwaitingInput,
        ItemMenuResult::NoResponse => RunState::ShowTargeting { range, item },
        ItemMenuResult::Selected => {
            gs.ecs
                .write_storage::<WantsToUseItem>()
                .insert(
                    *gs.ecs.fetch::<Entity>(),
                    WantsToUseItem {
                        item,
                        target: point,
                    },
                )
                .expect("Unable to insert intent");
            RunState::Ticking
        }
    }
}
