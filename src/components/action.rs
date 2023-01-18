use bracket_lib::terminal::Point;
use specs::prelude::*;
use specs_derive::*;

#[derive(Component)]
pub struct WantsToMelee {
    pub target: Entity,
}

#[derive(Component)]
pub struct WantsToPickupItem {
    pub collected_by: Entity,
    pub item: Entity,
}

#[derive(Component)]
pub struct WantsToUseItem {
    pub item: Entity,
    pub target: Option<Point>,
}

#[derive(Component)]
pub struct WantsToDropItem {
    pub item: Entity,
}

#[derive(Component)]
pub struct WantsToRemoveItem {
    pub item: Entity,
}
