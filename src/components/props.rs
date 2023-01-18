use bracket_lib::terminal::RGB;
use specs::prelude::*;
use specs_derive::*;

#[derive(Component)]
pub struct SingleActivation {}

#[derive(Component)]
pub struct Door {
    pub open: bool,
}

#[derive(Component)]
pub struct LightSource {
    pub color: RGB,
    pub range: i32,
}
