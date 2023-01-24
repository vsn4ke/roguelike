pub mod deep_cavern;

use super::{
    super::map::tiles::Surface,
    meta::{prefab_local::PrefabLocal, prefab_section::PrefabSection, X, Y},
};

pub struct Description {
    pub char: char,
    pub surface: Surface,
    pub name: String,
}

impl Description {
    #[allow(dead_code)]
    pub fn new(char: char, surface: Surface, name: &str) -> Self {
        Self {
            char,
            surface,
            name: name.to_string(),
        }
    }

    pub fn floor(char: char, name: &str) -> Self {
        Self {
            char,
            surface: Surface::Floor,
            name: name.to_string(),
        }
    }

    pub fn empty(char: char, surface: Surface) -> Self {
        Self {
            char,
            surface,
            name: "".to_string(),
        }
    }
}

impl Default for Description {
    fn default() -> Self {
        Self {
            char: ' ',
            surface: Surface::Floor,
            name: "".to_string(),
        }
    }
}

#[allow(dead_code)]
pub fn obvious_trap() -> (PrefabLocal, Vec<Description>) {
    let d: Vec<Description> = vec![
        Description::default(),
        Description::floor('^', "Bear trap"),
        Description::floor('!', "Health Potion"),
    ];

    let local = PrefabLocal {
        width: 5,
        height: 5,
        first_depth: 0,
        last_depth: 100,
        template: "
 ^^^
 ^!^
 ^^^
     ",
    };

    (local, d)
}

/*
use crate::map::tiles::Surface;


#[allow(dead_code)]
pub const CHECKERBOARD: PrefabRoom = PrefabRoom {
    width: 6,
    height: 5,
    first_depth: 0,
    last_depth: 100,
    template: "
    g# #
    #!#
    ^# #
           ",
    description: [
        (' ', Surface::Floor, "".to_string()),
        ('g', Surface::Floor, "Goblin".to_string()),
        ('#', Surface::Wall, "".to_string()),
        ('^', Surface::Floor, "Bear trap".to_string()),
        ('!', Surface::Floor, "Health Potion".to_string()),
    ]
    .to_vec(),
};
*/
