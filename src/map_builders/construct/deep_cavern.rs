use super::{
    prefabs, AreaStartingPosition, BuilderChain, CavernDecorator, DLABuilder, DistantExit,
    PrefabSectionBuilder, VoronoiSpawner, X, Y,
};

pub fn deep_cavern_builder(depth: i32, width: i32, height: i32) -> BuilderChain {
    let mut builder = BuilderChain::new(depth, width, height, "The Globin's Cavern");
    let (orc_camp_section, orc_camp_description) = prefabs::deep_cavern::orc_camp_prefab();
    builder
        .start_with(DLABuilder::central_attractor())
        .with(AreaStartingPosition::new((X::Left, Y::Top)))
        .with(VoronoiSpawner::new())
        .with(DistantExit::new())
        .with(CavernDecorator::new())
        .with(PrefabSectionBuilder::new(
            orc_camp_section,
            orc_camp_description,
        ));
    builder
}
