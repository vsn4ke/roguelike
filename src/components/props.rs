use specs::prelude::*;
use specs_derive::*;

#[derive(Component)]
pub struct SingleActivation {}

#[derive(Component)]
pub struct Door {
    pub open: bool,
}
