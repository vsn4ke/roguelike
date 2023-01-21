use super::{
    super::colors::*, particle::ParticleBuilder, AreaOfEffect, Confusion, Consumable,
    EquipmentChanged, Equippable, Equipped, InBackpack, InflictsDamage, Log, Map, Name, Pools,
    Position, ProvidesHealing, SufferDamage, WantsToDropItem, WantsToPickupItem, WantsToRemoveItem,
    WantsToUseItem,
};
use bracket_lib::{
    prelude::{field_of_view, Algorithm2D},
    terminal::to_cp437,
};
use specs::prelude::*;

pub struct ItemCollectionSystem {}

impl<'a> System<'a> for ItemCollectionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteStorage<'a, WantsToPickupItem>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, InBackpack>,
        WriteStorage<'a, EquipmentChanged>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, mut wants_pickup, mut positions, names, mut backpack, mut dirty) = data;

        for pickup in wants_pickup.join() {
            positions.remove(pickup.item);
            backpack
                .insert(
                    pickup.item,
                    InBackpack {
                        owner: pickup.collected_by,
                    },
                )
                .expect("Unable to insert backpack");
            dirty
                .insert(pickup.collected_by, EquipmentChanged {})
                .expect("Unable to insert EquipmentChanged");
            if pickup.collected_by == *player_entity {
                Log::new()
                    .append("You pick up the")
                    .item(&names.get(pickup.item).unwrap().name)
                    .build();
            }
        }
        wants_pickup.clear();
    }
}

pub struct ItemUseSystem {}

impl<'a> System<'a> for ItemUseSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        ReadExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, WantsToUseItem>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Pools>,
        ReadStorage<'a, Consumable>,
        ReadStorage<'a, ProvidesHealing>,
        ReadStorage<'a, InflictsDamage>,
        WriteStorage<'a, SufferDamage>,
        ReadStorage<'a, AreaOfEffect>,
        WriteStorage<'a, Confusion>,
        ReadStorage<'a, Equippable>,
        WriteStorage<'a, Equipped>,
        WriteStorage<'a, InBackpack>,
        WriteExpect<'a, ParticleBuilder>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, EquipmentChanged>,
    );

    #[allow(clippy::cognitive_complexity)]
    fn run(&mut self, data: Self::SystemData) {
        let (
            player_entity,
            map,
            entities,
            mut wants_use,
            names,
            mut combat_stats,
            consumables,
            healing,
            inflicts_damage,
            mut suffer_damage,
            area_of_effect,
            mut confused,
            equippable,
            mut equipped,
            mut backpack,
            mut particle_builder,
            positions,
            mut dirty,
        ) = data;

        for (entity, useitem) in (&entities, &wants_use).join() {
            let mut used_item = true;
            let mut targets: Vec<Entity> = Vec::new();
            dirty
                .insert(entity, EquipmentChanged {})
                .expect("Unable to insert EquipmentChanged");
            if let Some(target) = useitem.target {
                //AoE
                if let Some(aoe) = area_of_effect.get(useitem.item) {
                    let mut blast_tiles = field_of_view(target, aoe.radius, &*map);
                    blast_tiles.retain(|p| map.in_bounds(*p));
                    for tile_pt in blast_tiles.iter() {
                        let idx = map.point2d_to_index(*tile_pt);
                        for mob in map.tiles[idx].content.iter() {
                            targets.push(*mob);
                        }
                        particle_builder.request(
                            tile_pt.x,
                            tile_pt.y,
                            c(RED3),
                            c(BLACK),
                            to_cp437('░'),
                            200.0,
                        );
                    }
                } else {
                    //Mono
                    let idx = map.point2d_to_index(target);

                    for mob in map.tiles[idx].content.iter() {
                        targets.push(*mob);
                    }
                }
            } else {
                //Self cast
                targets.push(*player_entity);
            }

            if let Some(can_equip) = equippable.get(useitem.item) {
                let target_slot = can_equip.slot;
                let target = targets[0];

                let mut to_unequip: Vec<Entity> = Vec::new();
                for (item_entity, already_equipped, item) in (&entities, &equipped, &names).join() {
                    if already_equipped.owner == target && already_equipped.slot == target_slot {
                        to_unequip.push(item_entity);
                        if target == *player_entity {
                            Log::new().append("You unequip").item(&item.name).build();
                        }
                    }
                }

                for item in to_unequip.iter() {
                    equipped.remove(*item);
                    backpack
                        .insert(*item, InBackpack { owner: target })
                        .expect("Unable to insert into backpack");
                }

                equipped
                    .insert(
                        useitem.item,
                        Equipped {
                            owner: target,
                            slot: target_slot,
                        },
                    )
                    .expect("Unable to equipped component");
                backpack.remove(useitem.item);
                if target == *player_entity {
                    Log::new()
                        .append("You equip")
                        .item(&names.get(useitem.item).unwrap().name)
                        .build();
                }
            }

            if let Some(healer) = healing.get(useitem.item) {
                used_item = false;
                for target in targets.iter() {
                    if let Some(stats) = combat_stats.get_mut(*target) {
                        stats.hit_points.current = i32::min(
                            stats.hit_points.max,
                            stats.hit_points.current + healer.heal_amount,
                        );
                        if entity == *player_entity {
                            Log::new()
                                .item(&names.get(useitem.item).unwrap().name)
                                .append("heals you for")
                                .good(&healer.heal_amount)
                                .build();
                        }
                        used_item = true;

                        if let Some(pos) = positions.get(*target) {
                            particle_builder.request(
                                pos.x,
                                pos.y,
                                c(GREEN5),
                                c(BLACK),
                                to_cp437('♥'),
                                200.0,
                            );
                        }
                    }
                }
            }

            //damaging items
            if let Some(damage) = inflicts_damage.get(useitem.item) {
                used_item = false;
                for mob in targets.iter() {
                    SufferDamage::new_damage(&mut suffer_damage, *mob, damage.damage, true);
                    used_item = true;
                    if entity != *player_entity {
                        continue;
                    }

                    let mob_name = names.get(*mob).unwrap();
                    let item_name = names.get(useitem.item).unwrap();
                    Log::new()
                        .append("You use")
                        .item(&item_name.name)
                        .append("on")
                        .npc(&mob_name.name)
                        .append(", infilicting")
                        .bad(&damage.damage)
                        .append("damages")
                        .build();

                    let pos = positions.get(*mob);
                    if let Some(pos) = pos {
                        particle_builder.request(
                            pos.x,
                            pos.y,
                            c(RED1),
                            c(BLACK),
                            to_cp437('‼'),
                            200.0,
                        );
                    }
                }
            }

            let mut add_confusion = Vec::new();

            if let Some(confusion) = confused.get(useitem.item) {
                used_item = false;
                for mob in targets.iter() {
                    used_item = true;
                    add_confusion.push((*mob, confusion.duration));
                    if entity != *player_entity {
                        continue;
                    }

                    let mob_name = names.get(*mob).unwrap();
                    let item_name = names.get(useitem.item).unwrap();
                    Log::new()
                        .append("You use")
                        .item(&item_name.name)
                        .append("on")
                        .npc(&mob_name.name)
                        .build();

                    if let Some(pos) = positions.get(*mob) {
                        particle_builder.request(
                            pos.x,
                            pos.y,
                            c(GREEN4),
                            c(BLACK),
                            to_cp437('?'),
                            200.0,
                        );
                    }
                }
            }

            for mob in add_confusion.iter() {
                confused
                    .insert(mob.0, Confusion { duration: mob.1 })
                    .expect("Unable to insert confusion");
            }

            // remove used consumables
            if used_item && consumables.get(useitem.item).is_some() {
                entities.delete(useitem.item).expect("Delete failed");
            }
        }

        wants_use.clear();
    }
}

pub struct ItemDropSystem {}
impl<'a> System<'a> for ItemDropSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        Entities<'a>,
        WriteStorage<'a, WantsToDropItem>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, InBackpack>,
        WriteStorage<'a, EquipmentChanged>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player_entity,
            entities,
            mut want_drops,
            names,
            mut positions,
            mut backpack,
            mut dirty,
        ) = data;

        for (entity, to_drop) in (&entities, &want_drops).join() {
            let dropped_pos = positions.get(entity).unwrap();

            positions
                .insert(
                    to_drop.item,
                    Position {
                        x: dropped_pos.x,
                        y: dropped_pos.y,
                    },
                )
                .expect("Unable to insert positions");
            backpack.remove(to_drop.item);

            dirty
                .insert(entity, EquipmentChanged {})
                .expect("Unable to insert EquipmentChanged");

            if entity == *player_entity {
                Log::new()
                    .append("You drop the")
                    .item(&names.get(to_drop.item).unwrap().name)
                    .build();
            }
        }
        want_drops.clear();
    }
}

pub struct ItemRemoveSystem {}

impl<'a> System<'a> for ItemRemoveSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        Entities<'a>,
        WriteStorage<'a, WantsToRemoveItem>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Equipped>,
        WriteStorage<'a, InBackpack>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, entities, mut wants_remove, names, mut equipped, mut backpack) = data;

        for (entity, to_remove) in (&entities, &wants_remove).join() {
            equipped.remove(to_remove.item);
            backpack
                .insert(to_remove.item, InBackpack { owner: entity })
                .expect("Unable to insert in backpack");

            if entity == *player_entity {
                Log::new()
                    .append("You remove the")
                    .item(&names.get(to_remove.item).unwrap().name)
                    .build();
            }
        }
        wants_remove.clear();
    }
}
