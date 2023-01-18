use super::{
    get_renderable_component, spawn_position, BlocksTile, BlocksVisibility, Door, EntryTrigger,
    Hidden, InflictsDamage, Name, RawMaster, RenderableRaw, SingleActivation, SpawnType,
};
use serde::Deserialize;
use specs::prelude::*;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct PropRaw {
    pub name: String,
    pub renderable: Option<RenderableRaw>,
    pub hidden: Option<bool>,
    pub blocks_tile: Option<bool>,
    pub blocks_visibility: Option<bool>,
    pub door_open: Option<bool>,
    pub entry_trigger: Option<EntryTriggerRaw>,
}

#[derive(Deserialize)]
pub struct EntryTriggerRaw {
    pub effects: HashMap<String, String>,
}

pub fn spawn_named_prop(
    raws: &RawMaster,
    ecs: &mut World,
    key: &str,
    pos: SpawnType,
) -> Option<Entity> {
    if !raws.prop_index.contains_key(key) {
        return None;
    }

    let prop_template = &raws.raws.props[raws.prop_index[key]];
    let mut eb = ecs.create_entity();

    eb = spawn_position(pos, eb, key, raws);

    if let Some(renderable) = &prop_template.renderable {
        eb = eb.with(get_renderable_component(renderable));
    }

    eb = eb.with(Name {
        name: prop_template.name.clone(),
    });

    if let Some(hidden) = prop_template.hidden {
        if hidden {
            eb = eb.with(Hidden {})
        }
    }
    if let Some(blocks_tile) = prop_template.blocks_tile {
        if blocks_tile {
            eb = eb.with(BlocksTile {})
        }
    }
    if let Some(blocks_visibility) = prop_template.blocks_visibility {
        if blocks_visibility {
            eb = eb.with(BlocksVisibility {})
        }
    }
    if let Some(door_open) = prop_template.door_open {
        eb = eb.with(Door { open: door_open });
    }
    if let Some(entry_trigger) = &prop_template.entry_trigger {
        eb = eb.with(EntryTrigger {});
        for effect in entry_trigger.effects.iter() {
            match effect.0.as_str() {
                "damage" => {
                    eb = eb.with(InflictsDamage {
                        damage: effect.1.parse::<i32>().unwrap(),
                    })
                }
                "single_activation" => eb = eb.with(SingleActivation {}),
                _ => {}
            }
        }
    }

    Some(eb.build())
}
