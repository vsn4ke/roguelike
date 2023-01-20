use super::{
    super::colors::*, get_loots, particle::ParticleBuilder, spawn_named_item, Attributes, Log,
    LootTable, Map, Name, Player, Pools, Position, RunState, SpawnType, SufferDamage, RAWS,
};
use bracket_lib::terminal::{to_cp437, Point};
use specs::prelude::*;

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, Pools>,
        WriteStorage<'a, SufferDamage>,
        ReadStorage<'a, Position>,
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Attributes>,
        ReadExpect<'a, Entity>,
        ReadExpect<'a, Point>,
        WriteExpect<'a, ParticleBuilder>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut pools,
            mut damage,
            positions,
            mut map,
            entities,
            mut attr,
            player,
            ppos,
            mut particles,
        ) = data;

        let mut xp_gain = 0;
        let mut money_gain = 0;

        for (entity, mut pools, damage) in (&entities, &mut pools, &damage).join() {
            for dmg in damage.amount.iter() {
                pools.hit_points.current -= dmg.0;

                if pools.hit_points.current < 1 && dmg.1 {
                    if let Some(attr) = attr.get(entity) {
                        xp_gain += attr.level * 100;
                    }
                    money_gain += pools.money;
                }
            }

            if let Some(pos) = positions.get(entity) {
                let idx = map.coord_to_index(pos.x, pos.y);
                map.tiles[idx].bloodstains = true;
            }
        }

        if xp_gain > 0 || money_gain > 0 {
            let mut player_attr = attr.get_mut(*player).unwrap();
            let mut player_pools = pools.get_mut(*player).unwrap();

            player_pools.xp += xp_gain;
            player_pools.money += money_gain;

            let to_level = player_attr.level * 1000;

            if player_pools.xp >= to_level {
                // We've gone up a level!
                player_attr.level += 1;

                //log.entries.push(format!("Congratulations, you are now level {}", player_stats.level));

                player_pools.hit_points.max = player_attr.player_max_hp();
                player_pools.hit_points.current = player_pools.hit_points.max;

                player_pools.mana.max = player_attr.max_mana();
                player_pools.mana.current = player_pools.mana.max;

                player_pools.xp -= to_level;

                for i in 0..10 {
                    if ppos.y - i > 1 {
                        particles.request(
                            ppos.x,
                            ppos.y - i,
                            c(YELLOW3),
                            c(BLACK),
                            to_cp437('░'),
                            800.0,
                        );
                    }
                }
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
