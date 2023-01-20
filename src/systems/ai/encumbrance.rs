use super::{Attributes, EquipmentChanged, Equipped, InBackpack, Item, Log};
use specs::prelude::*;
use std::collections::HashMap;

pub struct EncumbranceSystem {}

impl<'a> System<'a> for EncumbranceSystem {
    type SystemData = (
        WriteStorage<'a, EquipmentChanged>,
        Entities<'a>,
        ReadStorage<'a, Item>,
        ReadStorage<'a, InBackpack>,
        ReadStorage<'a, Equipped>,
        WriteStorage<'a, Attributes>,
        ReadExpect<'a, Entity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut equip_dirty, entities, items, backpacks, wielded, mut attributes, player) = data;

        if equip_dirty.is_empty() {
            return;
        }

        let mut to_update: HashMap<Entity, (f32, f32)> = HashMap::new(); // (weight, intiative)
        for (entity, _) in (&entities, &equip_dirty).join() {
            to_update.insert(entity, (0.0, 0.0));
        }

        equip_dirty.clear();

        for (item, equipped) in (&items, &wielded).join() {
            if to_update.contains_key(&equipped.owner) {
                let totals = to_update.get_mut(&equipped.owner).unwrap();
                totals.0 += item.weight;
                totals.1 += item.initiative_penalty;
            }
        }

        for (item, carried) in (&items, &backpacks).join() {
            if to_update.contains_key(&carried.owner) {
                let totals = to_update.get_mut(&carried.owner).unwrap();
                totals.0 += item.weight;
                totals.1 += item.initiative_penalty;
            }
        }

        for (entity, (weight, initiative)) in to_update.iter() {
            if let Some(attr) = attributes.get_mut(*entity) {
                if *entity != *player {
                    continue;
                }

                attr.total_weight = *weight;
                attr.total_initiative_penalty = *initiative;

                if attr.total_weight > attr.max_weight() {
                    attr.total_initiative_penalty += 4.0;
                    Log::new()
                        .append("You are")
                        .bad(&"overburdened.")
                        .append("(initiative -4)")
                        .build();
                }
            }
        }
    }
}
