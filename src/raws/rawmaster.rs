use super::{
    c, items::find_slot_for_equippable_item, items::spawn_named_item, mobs::spawn_named_mob,
    props::spawn_named_prop, Equipped, InBackpack, Position, Raws, Renderable, RenderableRaw,
};
use bracket_lib::terminal::to_cp437;
use specs::prelude::*;
use std::collections::{HashMap, HashSet};

pub enum SpawnType {
    AtPosition { x: i32, y: i32 },
    Equipped { by: Entity },
    Carried { by: Entity },
}

pub struct RawMaster {
    pub raws: Raws,
    pub item_index: HashMap<String, usize>,
    pub mob_index: HashMap<String, usize>,
    pub prop_index: HashMap<String, usize>,
    pub loot_index: HashMap<String, usize>,
}

impl RawMaster {
    pub fn empty() -> RawMaster {
        RawMaster {
            raws: Raws {
                items: Vec::new(),
                mobs: Vec::new(),
                props: Vec::new(),
                spawn_tables: Vec::new(),
                loot_tables: Vec::new(),
            },
            item_index: HashMap::new(),
            mob_index: HashMap::new(),
            prop_index: HashMap::new(),
            loot_index: HashMap::new(),
        }
    }

    pub fn load(&mut self, raws: Raws) {
        self.raws = raws;
        self.item_index = HashMap::new();
        let mut used_names: HashSet<String> = HashSet::new();

        for (i, item) in self.raws.items.iter().enumerate() {
            if used_names.contains(&item.name) {
                println!("WARNING -  duplicate item name in raws [{}]", item.name);
            }
            self.item_index.insert(item.name.clone(), i);
            used_names.insert(item.name.clone());
        }
        for (i, mob) in self.raws.mobs.iter().enumerate() {
            if used_names.contains(&mob.name) {
                println!("WARNING -  duplicate mob name in raws [{}]", mob.name);
            }
            self.mob_index.insert(mob.name.clone(), i);
            used_names.insert(mob.name.clone());
        }
        for (i, prop) in self.raws.props.iter().enumerate() {
            if used_names.contains(&prop.name) {
                println!("WARNING -  duplicate prop name in raws [{}]", prop.name);
            }
            self.prop_index.insert(prop.name.clone(), i);
            used_names.insert(prop.name.clone());
        }

        for (i, loot) in self.raws.loot_tables.iter().enumerate() {
            self.loot_index.insert(loot.name.clone(), i);
        }

        for spawn in self.raws.spawn_tables.iter() {
            if !used_names.contains(&spawn.name) {
                println!(
                    "WARNING -  Spawn tables references unspecified entity {}",
                    spawn.name
                );
            }
        }
    }
}

pub fn spawn_named_entity(
    raws: &RawMaster,
    ecs: &mut World,
    key: &str,
    pos: SpawnType,
) -> Option<Entity> {
    if raws.item_index.contains_key(key) {
        return spawn_named_item(raws, ecs, key, pos);
    } else if raws.mob_index.contains_key(key) {
        return spawn_named_mob(raws, ecs, key, pos);
    } else if raws.prop_index.contains_key(key) {
        return spawn_named_prop(raws, ecs, key, pos);
    }
    None
}

pub fn spawn_position<'a>(
    spawn: SpawnType,
    entity: EntityBuilder<'a>,
    tag: &str,
    raws: &RawMaster,
) -> EntityBuilder<'a> {
    match spawn {
        SpawnType::AtPosition { x, y } => entity.with(Position { x, y }),
        SpawnType::Carried { by } => entity.with(InBackpack { owner: by }),
        SpawnType::Equipped { by } => entity.with(Equipped {
            owner: by,
            slot: find_slot_for_equippable_item(tag, raws),
        }),
    }
}

pub fn get_renderable_component(renderable: &RenderableRaw) -> Renderable {
    Renderable {
        glyph: to_cp437(renderable.glyph.chars().next().unwrap()),
        fg: c(&renderable.fg),
        bg: c(&renderable.bg),
        render_order: renderable.order,
    }
}
