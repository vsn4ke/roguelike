use super::{
    get_loots, spawn_named_item, Log, LootTable, Map, Name, Player, Pools, Position, RunState,
    SpawnType, SufferDamage, RAWS,
};
use specs::prelude::*;

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, Pools>,
        WriteStorage<'a, SufferDamage>,
        ReadStorage<'a, Position>,
        WriteExpect<'a, Map>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut stats, mut damage, positions, mut map, entities) = data;

        for (entity, mut stats, damage) in (&entities, &mut stats, &damage).join() {
            stats.hit_points.current -= damage.amount.iter().sum::<i32>();

            if let Some(pos) = positions.get(entity) {
                let idx = map.coord_to_index(pos.x, pos.y);
                map.tiles[idx].bloodstains = true;
            }
        }

        damage.clear();
    }
}

pub fn delete_the_deads(ecs: &mut World) {
    let mut dead: Vec<Entity> = Vec::new();

    {
        let entities = ecs.entities();
        let combat_stats = ecs.read_storage::<Pools>();
        let players = ecs.read_storage::<Player>();
        let names = ecs.read_storage::<Name>();

        for (entity, stats) in (&entities, &combat_stats).join() {
            if stats.hit_points.current >= 1 {
                continue;
            }
            if players.get(entity).is_some() {
                let mut runstate = ecs.write_resource::<RunState>();
                *runstate = RunState::GameOver;
            } else {
                if let Some(victim) = names.get(entity) {
                    Log::new().npc(&victim.name).append("is dead.").build();
                }
                dead.push(entity)
            }
        }
    }

    let mut to_spawn: Vec<(String, Position)> = Vec::new();

    for victim in dead.iter() {
        if let Some(loot) = ecs.read_storage::<LootTable>().get(*victim) {
            if let Some(tag) = get_loots(&RAWS.lock().unwrap(), &loot.table) {
                if let Some(pos) = ecs.write_storage::<Position>().get(*victim) {
                    to_spawn.push((tag, *pos));
                }
            }
        }

        ecs.delete_entity(*victim).expect("Unable to delete");
    }

    for drop in to_spawn.iter() {
        spawn_named_item(
            &RAWS.lock().unwrap(),
            ecs,
            &drop.0,
            SpawnType::AtPosition {
                x: drop.1.x,
                y: drop.1.y,
            },
        );
    }
}
