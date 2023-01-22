use super::{
    random_position, AreaStartingPosition, BSPCorridors, BspDungeonBuilder, BspInteriorBuilder,
    BuilderChain, CellularAutomataBuilder, CorridorSpawner, CullUnreachable, DLABuilder,
    DistantExit, DoglegCorridors, DoorPlacement, DrunkardsWalkBuilder, MazeBuilder,
    NearestCorridors, Nothing, RandomGen, RoomBasedSpawner, RoomBasedStairs,
    RoomBasedStartingPosition, RoomCornerRounder, RoomDrawer, RoomExploder, RoomSorter,
    SimpleMapBuilder, Sorter, StraightLineCorridors, VoronoiCellBuilder, VoronoiSpawner, X, Y,
};

pub fn random_builder(depth: i32, width: i32, height: i32) -> BuilderChain {
    let mut rng = RandomGen::default();
    let mut builder = BuilderChain::new(depth, width, height, "Random dungeon");

    match rng.range(0, 2) {
        1 => random_room_builder(&mut builder),
        _ => random_shape_builder(&mut builder),
    }
    builder.with(DoorPlacement::new());

    builder
}

fn random_shape_builder(builder: &mut BuilderChain) {
    let mut rng = RandomGen::default();
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
    let mut rng = RandomGen::default();
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
