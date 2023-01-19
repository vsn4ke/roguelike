use std::collections::HashMap;

use bracket_lib::{
    random::RandomNumberGenerator,
    terminal::{to_cp437, Rect},
};
use specs::prelude::*;

const MAX_SPAWNS: i32 = 6;

use super::{
    colors::*,
    gamesystem::{player_hp, total_mana},
    map::{tiles::Surface, Map},
    props::LightSource,
    raws::{spawn_named_entity, spawn_table::get_spawn_table_for_depth, SpawnType, RAWS},
    unit::{
        Attribute, Attributes, Faction, Initiative, Player, Pool, Pools, Skill, Skills, Viewshed,
    },
    Name, Position, Renderable,
};

pub mod random_table;

pub fn spawn_in_room(map: &Map, room: &Rect, spawn_list: &mut Vec<(usize, String)>) {
    let mut possible_targets: Vec<usize> = Vec::new();

    for y in room.y1..room.y2 {
        for x in room.x1..room.y2 {
            let idx = map.coord_to_index(x, y);
            if map.tiles[idx].surface == Surface::Floor {
                possible_targets.push(idx);
            }
        }
    }

    spawn_in_region(&possible_targets, map.depth, spawn_list);
}

pub fn spawn_in_region(area: &[usize], depth: i32, spawn_list: &mut Vec<(usize, String)>) {
    let spawn_table = get_spawn_table_for_depth(&RAWS.lock().unwrap(), depth);
    let mut rng = RandomNumberGenerator::new();
    let mut spawn_points: HashMap<usize, String> = HashMap::new();
    let mut areas: Vec<usize> = Vec::from(area);
    {
        let num_spawns = i32::min(areas.len() as i32, rng.range(-2, MAX_SPAWNS - 2) + depth);
        if num_spawns <= 0 {
            return;
        }

        for _ in 0..num_spawns {
            let array_index = rng.range(0, areas.len());
            let map_idx = areas[array_index];
            spawn_points.insert(map_idx, spawn_table.roll());
            areas.remove(array_index);
        }
    }

    for spawn in spawn_points.iter() {
        spawn_list.push((*spawn.0, spawn.1.to_string()));
    }
}

pub fn spawn_entity(ecs: &mut World, spawn: &(&usize, &String)) {
    let map = ecs.fetch::<Map>();
    let x = *spawn.0 as i32 % map.width;
    let y = *spawn.0 as i32 / map.width;
    std::mem::drop(map);

    let spawn_result = spawn_named_entity(
        &RAWS.lock().unwrap(),
        ecs,
        spawn.1,
        SpawnType::AtPosition { x, y },
    );
    if spawn_result.is_some() {
        return;
    }

    if spawn.1 != "None" {
        println!("WARNING: We don't know how to spawn [{:?}]!", spawn);
    }
}

pub fn build_player_entity(ecs: &mut World, x: i32, y: i32) -> Entity {
    let mut skills = Skills {
        skills: HashMap::new(),
    };
    skills.skills.insert(Skill::Defense, 1);
    skills.skills.insert(Skill::Melee, 1);
    skills.skills.insert(Skill::Magic, 1);

    let player = ecs
        .create_entity()
        .with(Position::new(x, y))
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: c(YELLOW1),
            bg: c(BLACK),
            render_order: 0,
        })
        .with(Player {})
        .with(Viewshed::new(8))
        .with(Name::new("Player"))
        .with(Attributes {
            might: Attribute {
                base: 11,
                modifiers: 0,
                bonus: 0,
            },
            fitness: Attribute {
                base: 11,
                modifiers: 0,
                bonus: 0,
            },
            quickness: Attribute {
                base: 11,
                modifiers: 0,
                bonus: 0,
            },
            intelligence: Attribute {
                base: 11,
                modifiers: 0,
                bonus: 0,
            },
        })
        .with(Pools {
            hit_points: Pool {
                current: player_hp(11, 1),
                max: player_hp(11, 1),
            },
            mana: Pool {
                current: total_mana(11, 1),
                max: total_mana(11, 1),
            },
            xp: 0,
            level: 1,
        })
        .with(skills)
        .with(LightSource {
            color: c(YELLOW5),
            range: 8,
        })
        .with(Initiative { current: 0 })
        .with(Faction {
            name: "Player".to_string(),
        })
        .build();

    spawn_named_entity(
        &RAWS.lock().unwrap(),
        ecs,
        "Rusty Longsword",
        SpawnType::Equipped { by: player },
    );

    spawn_named_entity(
        &RAWS.lock().unwrap(),
        ecs,
        "Stained Tunic",
        SpawnType::Equipped { by: player },
    );
    spawn_named_entity(
        &RAWS.lock().unwrap(),
        ecs,
        "Torn Trousers",
        SpawnType::Equipped { by: player },
    );
    spawn_named_entity(
        &RAWS.lock().unwrap(),
        ecs,
        "Old Boots",
        SpawnType::Equipped { by: player },
    );

    spawn_named_entity(
        &RAWS.lock().unwrap(),
        ecs,
        "Health Potion",
        SpawnType::Carried { by: player },
    );
    spawn_named_entity(
        &RAWS.lock().unwrap(),
        ecs,
        "Health Potion",
        SpawnType::Carried { by: player },
    );

    player
}
