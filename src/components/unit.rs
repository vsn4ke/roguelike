use std::collections::HashMap;

use bracket_lib::terminal::Point;
use specs::prelude::*;
use specs_derive::*;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Bystander {}

#[derive(Component)]
pub struct Vendor {}

#[derive(Component)]
pub struct Monster {}

#[derive(Component)]
pub struct Herbivore {}

#[derive(Component)]
pub struct Carnivore {}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

impl Viewshed {
    pub fn new(r: i32) -> Viewshed {
        Viewshed {
            visible_tiles: Vec::new(),
            range: r,
            dirty: true,
        }
    }
}

#[derive(Component)]
pub struct Quips {
    pub available: Vec<String>,
}

#[derive(Component)]
pub struct SufferDamage {
    pub amount: Vec<i32>,
}

impl SufferDamage {
    pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push(amount);
        } else {
            let damage = SufferDamage {
                amount: vec![amount],
            };
            store
                .insert(victim, damage)
                .expect("Unable to insert damage");
        }
    }
}

#[derive(Component)]
pub struct EntityMoved {}

#[derive(Component, Clone, Copy)]
pub struct Attribute {
    pub base: i32,
    pub modifiers: i32,
    pub bonus: i32,
}

#[derive(Component, Clone, Copy)]
pub struct Attributes {
    pub might: Attribute,
    pub fitness: Attribute,
    pub quickness: Attribute,
    pub intelligence: Attribute,
}

#[derive(Component)]
pub struct Skills {
    pub skills: HashMap<Skill, i32>,
}

#[derive(Hash, PartialEq, Eq)]
pub enum Skill {
    Melee,
    Defense,
    Magic,
}

#[derive(Component)]
pub struct Pool {
    pub max: i32,
    pub current: i32,
}

#[derive(Component)]
pub struct Pools {
    pub hit_points: Pool,
    pub mana: Pool,
    pub xp: i32,
    pub level: i32,
}

#[derive(Component)]
pub struct NaturalAttack {
    pub name: String,
    pub damage_n_dice: i32,
    pub damage_die_type: i32,
    pub damage_bonus: i32,
    pub hit_bonus: i32,
}

#[derive(Component)]
pub struct NaturalProperty {
    pub armor_class: Option<i32>,
    pub attacks: Vec<NaturalAttack>,
}

#[derive(Component)]
pub struct LootTable {
    pub table: String,
}
