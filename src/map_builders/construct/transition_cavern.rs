use super::{
    AreaEndingPosition, AreaStartingPosition, BspDungeonBuilder, BuilderChain, BuilderMap,
    CavernDecorator, CellularAutomataBuilder, CullUnreachable, DistantExit, NearestCorridors,
    RoomBasedSpawner, RoomDrawer, RoomExploder, RoomSorter, Sorter, VoronoiSpawner, X, Y,
};

pub fn transition_cavern_builder(depth: i32, width: i32, height: i32) -> BuilderChain {
    let mut builder = BuilderChain::new(depth, width, height, "Into the fort");

    builder
        .start_with(CellularAutomataBuilder::new())
        .with(AreaStartingPosition::new((X::Center, Y::Center)))
        .with(CullUnreachable::new())
        .with(AreaStartingPosition::new((X::Left, Y::Center)))
        .with(VoronoiSpawner::new())
        .with(CavernDecorator::new())
        .with(CavernTransitionBuilder::new(depth))
        .with(AreaStartingPosition::new((X::Left, Y::Center)))
        .with(CullUnreachable::new())
        .with(AreaEndingPosition::new((X::Right, Y::Center)))
        .with(DistantExit::new());
    builder
}

pub struct CavernTransitionBuilder {
    depth: i32,
}

impl super::MetaMapBuilder for CavernTransitionBuilder {
    fn build_map(&mut self, data: &mut BuilderMap) {
        let mut builder = BuilderChain::new(self.depth, data.width, data.height, "New Map");
        builder
            .start_with(BspDungeonBuilder::new())
            .with(RoomDrawer::new())
            .with(RoomSorter::new(Sorter::RightMost))
            .with(NearestCorridors::new())
            .with(RoomExploder::new())
            .with(RoomBasedSpawner::new())
            .build_map();

        for h in builder.data.history.iter() {
            data.history.push(h.clone());
        }
        data.take_snapshot();

        for x in data.map.width / 2..data.map.width {
            for y in 0..data.map.height {
                let idx = data.map.coord_to_index(x, y);
                data.map.tiles[idx] = builder.data.map.tiles[idx].clone();
            }
        }
        data.take_snapshot();

        let w = data.map.width;
        data.spawn_list.retain(|s| s.0 as i32 / w < w / 2);

        for s in builder.data.spawn_list.iter() {
            if s.0 as i32 / w > w / 2 {
                data.spawn_list.push(s.clone());
            }
        }
    }
}

impl CavernTransitionBuilder {
    pub fn new(depth: i32) -> Box<Self> {
        Box::new(Self { depth })
    }
}
