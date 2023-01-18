use bracket_lib::terminal::Point;
use bracket_terminal::{prelude::RGB, FontCharType};
use specs::prelude::*;
use specs_derive::*;
pub mod action;
pub mod effect;
pub mod item;
pub mod props;
pub mod unit;

#[derive(Component)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32,
}

#[derive(Component)]
pub struct Name {
    pub name: String,
}

impl Name {
    pub fn new(n: &str) -> Name {
        Name {
            name: n.to_string(),
        }
    }
}

#[derive(Component)]
pub struct BlocksTile {}

#[derive(Component)]
pub struct ParticleLifetime {
    pub lifetime_ms: f32,
}

#[derive(Component)]
pub struct Hidden {}

#[derive(Component)]
pub struct BlocksVisibility {}

#[derive(Component, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new<T>(x: T, y: T) -> Position
    where
        T: TryInto<i32>,
    {
        Position {
            x: x.try_into().ok().unwrap(),
            y: y.try_into().ok().unwrap(),
        }
    }

    pub fn into_point(&self) -> Point {
        Point::new(self.x, self.y)
    }

    pub fn from_point(p: Point) -> Position {
        Position { x: p.x, y: p.y }
    }
}

#[derive(Component, Clone, Copy)]
pub struct OtherLevelPosition {
    pub x: i32,
    pub y: i32,
    pub depth: i32,
}
