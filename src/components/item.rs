use specs::prelude::*;
use specs_derive::*;

#[derive(Component)]
pub struct Item {
    pub initiative_penalty: f32,
    pub weight: f32,
    pub base_value: i32,
}

#[derive(Component)]
pub struct InBackpack {
    pub owner: Entity,
}

#[derive(Component)]
pub struct Consumable {}

#[derive(PartialEq, Copy, Clone)]
pub enum EquipmentSlot {
    Melee,
    Shield,
    Hands,
    Head,
    Legs,
    Feet,
    Torso,
}

#[derive(Component)]
pub struct Equippable {
    pub slot: EquipmentSlot,
}

#[derive(Component, PartialEq)]
pub struct Equipped {
    pub owner: Entity,
    pub slot: EquipmentSlot,
}

#[derive(Clone, Copy)]
pub enum WeaponAttribute {
    Might,
    Quickness,
}

#[derive(Component, Clone, Copy)]
pub struct MeleeWeapon {
    pub attribute: WeaponAttribute,
    pub damage_n_dice: i32,
    pub damage_die_type: i32,
    pub damage_bonus: i32,
    pub hit_bonus: i32,
}

impl MeleeWeapon {
    pub fn base() -> MeleeWeapon {
        MeleeWeapon {
            attribute: WeaponAttribute::Might,
            damage_n_dice: 1,
            damage_die_type: 4,
            damage_bonus: 0,
            hit_bonus: 0,
        }
    }
}

#[derive(Component)]
pub struct Wearable {
    pub armor_class: i32,
    pub slot: EquipmentSlot,
}

#[derive(Component)]
pub struct EquipmentChanged {}
