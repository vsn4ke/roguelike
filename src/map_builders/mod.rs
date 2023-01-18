use super::{spawner, Map, SHOW_MAPGEN_VISUALIZER};
use bracket_lib::random::RandomNumberGenerator;
use bracket_lib::terminal::{Point, Rect};
use specs::World;

mod initial;
mod meta;

use initial::bsp_dungeon::BspDungeonBuilder;
use initial::bsp_interior::BspInteriorBuilder;
use initial::cellular_automata::CellularAutomataBuilder;
use initial::dla::DLABuilder;
use initial::drunkard::DrunkardsWalkBuilder;
//use initial::forest::ForestBuilder;
use initial::maze::MazeBuilder;
use initial::simple_map::SimpleMapBuilder;
use initial::town::TownBuilder;
use initial::voronoi::VoronoiCellBuilder;

use meta::area_starting_position::AreaStartingPosition;
use meta::corridor_spawner::CorridorSpawner;
use meta::cull_unreachable::CullUnreachable;
use meta::distant_exit::DistantExit;
use meta::door_placement::DoorPlacement;
use meta::forest_road::ForestRoad;
use meta::nothing::Nothing;
use meta::room_based_spawner::RoomBasedSpawner;
use meta::room_based_stairs::RoomBasedStairs;
use meta::room_based_starting_position::RoomBasedStartingPosition;
use meta::room_corner_rounding::RoomCornerRounder;
use meta::room_draw::RoomDrawer;
use meta::room_exploder::RoomExploder;
use meta::room_sorter::{RoomSorter, Sorter};
use meta::rooms_corridor_dogleg::DoglegCorridors;
use meta::rooms_corridors_bsp::BSPCorridors;
use meta::rooms_corridors_lines::StraightLineCorridors;
use meta::rooms_corridors_nearest::NearestCorridors;
use meta::voronoi_spawner::VoronoiSpawner;
use meta::{random_position, X, Y};

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
        _ => random_builder(depth, width, height),
    }
}

pub fn town_builder(depth: i32, width: i32, height: i32) -> BuilderChain {
    let mut builder = BuilderChain::new(depth, width, height, "Town of Lost Hope");
    builder.start_with(TownBuilder::new());

    builder
}

pub fn forest_builder(depth: i32, width: i32, height: i32) -> BuilderChain {
    let mut builder = BuilderChain::new(depth, width, height, "The Deep Dark Forest");
    builder
        .start_with(CellularAutomataBuilder::new())
        .with(AreaStartingPosition::new((X::Center, Y::Center)))
        .with(CullUnreachable::new())
        .with(AreaStartingPosition::new((X::Left, Y::Center)))
        .with(VoronoiSpawner::new())
        .with(ForestRoad::new());

    builder
}

pub fn random_builder(depth: i32, width: i32, height: i32) -> BuilderChain {
    let mut rng = RandomNumberGenerator::new();
    let mut builder = BuilderChain::new(depth, width, height, "Random dungeon");

    match rng.range(0, 2) {
        1 => random_room_builder(&mut builder),
        _ => random_shape_builder(&mut builder),
    }
    builder.with(DoorPlacement::new());

    builder
}

fn random_shape_builder(builder: &mut BuilderChain) {
    let mut rng = RandomNumberGenerator::new();
    builder
        .start_with(match rng.range(1, 14) {
            1 => CellularAutomataBuilder::new(),
            2 => DrunkardsWalkBuilder::open_area(),
            3 => DrunkardsWalkBuilder::open_halls(),
            4 => DrunkardsWalkBuilder::winding_passages(),
            5 => DrunkardsWalkBuilder::fat_passages(),
            6 => DrunkardsWalkBuilder::fearful_symmetry(),
            7 => MazeBuilder::new(),
            8 => DLABuilder::walk_inwards(),
            9 => DLABuilder::walk_outwards(),
            10 => DLABuilder::central_attractor(),
            11 => DLABuilder::insectoid(),
            12 => VoronoiCellBuilder::pythagoras(),
            _ => VoronoiCellBuilder::manhattan(),
        })
        .with(AreaStartingPosition::new((X::Center, Y::Center)))
        .with(CullUnreachable::new())
        .with(AreaStartingPosition::new(random_position()))
        .with(VoronoiSpawner::new())
        .with(DistantExit::new());
}

fn random_room_builder(builder: &mut BuilderChain) {
    let mut rng = RandomNumberGenerator::new();
    let roll = rng.range(0, 3);
    builder.start_with(match roll {
        0 => SimpleMapBuilder::new(),
        1 => BspDungeonBuilder::new(),
        _ => BspInteriorBuilder::new(),
    });

    if roll != 2 {
        builder
            .with(RoomDrawer::new())
            .with(RoomSorter::new(match rng.range(0, 5) {
                0 => Sorter::LeftMost,
                1 => Sorter::RightMost,
                2 => Sorter::TopMost,
                3 => Sorter::BottomMost,
                _ => Sorter::Central,
            }))
            .with(match rng.range(0, 4) {
                0 => StraightLineCorridors::new(),
                1 => NearestCorridors::new(),
                2 => DoglegCorridors::new(),
                _ => BSPCorridors::new(),
            })
            .with(match rng.range(0, 6) {
                1 => RoomExploder::new(),
                2 => RoomCornerRounder::new(),
                _ => Nothing::new(),
            })
            .with(match rng.range(0, 2) {
                0 => CorridorSpawner::new(),
                _ => Nothing::new(),
            });
    }

    builder
        .with(match rng.range(0, 2) {
            0 => RoomBasedStartingPosition::new(),
            _ => AreaStartingPosition::new(random_position()),
        })
        .with(match rng.range(0, 2) {
            0 => RoomBasedStairs::new(),
            _ => DistantExit::new(),
        })
        .with(match rng.range(0, 2) {
            0 => RoomBasedSpawner::new(),
            _ => VoronoiSpawner::new(),
        });
}
