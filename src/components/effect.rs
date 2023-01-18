use specs::prelude::*;
use specs_derive::*;

#[derive(Component)]
pub struct Ranged {
    pub range: i32,
}

#[derive(Component)]
pub struct ProvidesHealing {
    pub heal_amount: i32,
}

#[derive(Component)]
pub struct InflictsDamage {
    pub damage: i32,
}

#[derive(Component)]
pub struct AreaOfEffect {
    pub radius: i32,
}

#[derive(Component)]
pub struct Confusion {
    pub duration: i32,
}

#[derive(Component)]
pub struct EntryTrigger {}
