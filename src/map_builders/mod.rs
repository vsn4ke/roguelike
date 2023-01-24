use super::{rng, spawner, Map, SHOW_MAPGEN_VISUALIZER};
use bracket_lib::terminal::{Point, Rect};
use specs::World;

mod construct;
mod initial;
mod meta;
mod prefabs;

use construct::{
    cavern::cavern_builder, deep_cavern::deep_cavern_builder, forest::forest_builder,
    random::random_builder, town::town_builder, transition_cavern::transition_cavern_builder,
};

pub struct BuilderMap {
    pub spawn_list: Vec<(usize, String)>,
    pub map: Map,
    pub starting_point: Option<Point>,
    pub rooms: Option<Vec<Rect>>,
    pub history: Vec<Map>,
    pub corridors: Option<Vec<Vec<usize>>>,
    pub width: i32,
    pub height: i32,
}

impl BuilderMap {
    fn take_snapshot(&mut self) {
        if SHOW_MAPGEN_VISUALIZER {
            let mut snapshot = self.map.clone();
            for v in snapshot.tiles.iter_mut() {
                v.revealed = true;
            }
            self.history.push(snapshot);
        }
    }
}

pub struct BuilderChain {
    starter: Option<Box<dyn InitialMapBuilder>>,
    builders: Vec<Box<dyn MetaMapBuilder>>,
    pub data: BuilderMap,
}

impl BuilderChain {
    pub fn new<S: ToString>(depth: i32, width: i32, height: i32, name: S) -> BuilderChain {
        BuilderChain {
            starter: None,
            builders: Vec::new(),
            data: BuilderMap {
                spawn_list: Vec::new(),
                map: Map::new(depth, width, height, name),
                starting_point: None,
                rooms: None,
                history: Vec::new(),
                corridors: None,
                width,
                height,
            },
        }
    }

    pub fn start_with(&mut self, starter: Box<dyn InitialMapBuilder>) -> &mut BuilderChain {
        match self.starter {
            None => self.starter = Some(starter),
            Some(_) => panic!("Only one starter builder allowed."),
        }
        self
    }

    #[allow(dead_code)]
    pub fn with(&mut self, metabuilder: Box<dyn MetaMapBuilder>) -> &mut BuilderChain {
        self.builders.push(metabuilder);
        self
    }

    pub fn build_map(&mut self) {
        match &mut self.starter {
            None => panic!("A starter builder is required first."),
            Some(starter) => {
                starter.build_map(&mut self.data);
            }
        }

        for metabuilder in self.builders.iter_mut() {
            metabuilder.build_map(&mut self.data);
        }
    }

    pub fn spawn_entities(&mut self, ecs: &mut World) {
        for e in self.data.spawn_list.iter() {
            spawner::spawn_entity(ecs, &(&e.0, &e.1));
        }
    }
}

pub trait InitialMapBuilder {
    fn build_map(&mut self, data: &mut BuilderMap);
}

pub trait MetaMapBuilder {
    fn build_map(&mut self, data: &mut BuilderMap);
}

pub fn level_builder(depth: i32, width: i32, height: i32) -> BuilderChain {
    match depth {
        0 => town_builder(depth, 80, 50),
        1 => forest_builder(depth, 100, 60),
        2 => cavern_builder(depth, 100, 60),
        3 => deep_cavern_builder(depth, 80, 80),
        4 => transition_cavern_builder(depth, 80, 80),
        _ => random_builder(depth, width, height),
    }
}
