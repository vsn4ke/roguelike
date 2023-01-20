use bracket_lib::random::parse_dice_string;
use serde::Deserialize;
use specs::prelude::*;
use std::collections::HashMap;

use super::{
    rawmaster::{get_renderable_component, spawn_position},
    AreaOfEffect, Confusion, Consumable, EquipmentSlot, Equippable, InflictsDamage, Item,
    MeleeWeapon, Name, ProvidesHealing, Ranged, RawMaster, RenderableRaw, SpawnType,
    WeaponAttribute, Wearable,
};

#[derive(Deserialize)]
pub struct ItemRaw {
    pub name: String,
    pub renderable: Option<RenderableRaw>,
    pub consumable: Option<ConsumableRaw>,
    pub weapon: Option<WeaponRaw>,
    pub wearable: Option<WearableRaw>,
    pub initiative_penalty: Option<f32>,
    pub weight_kg: Option<f32>,
    pub base_value: Option<i32>,
}

#[derive(Deserialize)]
pub struct ConsumableRaw {
    pub effects: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct WeaponRaw {
    pub range: String,
    pub attribute: String,
    pub base_damage: String,
    pub hit_bonus: i32,
}

#[derive(Deserialize)]
pub struct WearableRaw {
    pub armor_class: i32,
    pub slot: String,
}

pub fn spawn_named_item(
    raws: &RawMaster,
    ecs: &mut World,
    key: &str,
    pos: SpawnType,
) -> Option<Entity> {
    if !raws.item_index.contains_key(key) {
        return None;
    }

    let item_template = &raws.raws.items[raws.item_index[key]];
    let mut eb = ecs.create_entity();
    eb = spawn_position(pos, eb, key, raws);

    if let Some(renderable) = &item_template.renderable {
        eb = eb.with(get_renderable_component(renderable));
    }

    eb = eb.with(Name {
        name: item_template.name.clone(),
    });

    eb = eb.with(Item {
        initiative_penalty: item_template.initiative_penalty.unwrap_or(0.0),
        weight: item_template.weight_kg.unwrap_or(0.0),
        base_value: item_template.base_value.unwrap_or(0),
    });

    if let Some(consumable) = &item_template.consumable {
        eb = eb.with(Consumable {});
        for effect in consumable.effects.iter() {
            let effect_name = effect.0.as_str();
            match effect_name {
                "provides_healing" => {
                    eb = eb.with(ProvidesHealing {
                        heal_amount: effect.1.parse::<i32>().unwrap(),
                    })
                }
                "ranged" => {
                    eb = eb.with(Ranged {
                        range: effect.1.parse::<i32>().unwrap(),
                    })
                }
                "damage" => {
                    eb = eb.with(InflictsDamage {
                        damage: effect.1.parse::<i32>().unwrap(),
                    })
                }
                "area_of_effect" => {
                    eb = eb.with(AreaOfEffect {
                        radius: effect.1.parse::<i32>().unwrap(),
                    })
                }
                "confusion" => {
                    eb = eb.with(Confusion {
                        duration: effect.1.parse::<i32>().unwrap(),
                    })
                }
                _ => {
                    println!(
                        "Warning: consumable effect {} not implemented.",
                        effect_name
                    );
                }
            }
        }
    }
    if let Some(weapon) = &item_template.weapon {
        eb = eb.with(Equippable {
            slot: EquipmentSlot::Melee,
        });
        let dice = parse_dice_string(&weapon.base_damage).unwrap();

        eb = eb.with(MeleeWeapon {
            attribute: match weapon.attribute.as_str() {
                "Quickness" => WeaponAttribute::Quickness,
                _ => WeaponAttribute::Might,
            },
            damage_n_dice: dice.n_dice,
            damage_bonus: dice.bonus,
            damage_die_type: dice.die_type,
            hit_bonus: weapon.hit_bonus,
        });
    }

    if let Some(wearable) = &item_template.wearable {
        let slot = string_to_slot(&wearable.slot);
        eb = eb.with(Equippable { slot });
        eb = eb.with(Wearable {
            slot,
            armor_class: wearable.armor_class,
        });
    }

    Some(eb.build())
}

pub fn find_slot_for_equippable_item(tag: &str, raws: &RawMaster) -> EquipmentSlot {
    if !raws.item_index.contains_key(tag) {
        panic!("Trying to equip an unknown item: {}", tag);
    }

    let idx = raws.item_index[tag];
    let item = &raws.raws.items[idx];
    if item.weapon.is_some() {
        return EquipmentSlot::Melee;
    } else if let Some(wearable) = &item.wearable {
        return string_to_slot(&wearable.slot);
    }

    panic!("Trying to equip {}, but it has no slot tag.", tag);
}

pub fn string_to_slot(slot: &str) -> EquipmentSlot {
    match slot {
        "Shield" => EquipmentSlot::Shield,
        "Melee" => EquipmentSlot::Melee,
        "Head" => EquipmentSlot::Head,
        "Torso" => EquipmentSlot::Torso,
        "Legs" => EquipmentSlot::Legs,
        "Feet" => EquipmentSlot::Feet,
        "Hands" => EquipmentSlot::Hands,
        _ => {
            println!("Warning: unknown equipment slot type [{}])", slot);
            EquipmentSlot::Melee
        }
    }
}
