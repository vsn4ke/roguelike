#[derive(Clone, Copy, PartialEq, Default)]
pub enum Surface {
    #[default]
    Wall,
    Floor,
    DownStairs,
    Grass,
    DeepWater,
    ShallowWater,
    Bridge,
    Road,
    Gravel,
    WoodFloor,
    Path,
    UpStairs,
}

pub fn is_tile_walkable(tile_type: Surface) -> bool {
    matches!(
        tile_type,
        Surface::Floor
            | Surface::DownStairs
            | Surface::Road
            | Surface::Grass
            | Surface::ShallowWater
            | Surface::WoodFloor
            | Surface::Bridge
            | Surface::Gravel
            | Surface::Path
            | Surface::UpStairs
    )
}

pub fn is_tile_opaque(tile_type: Surface) -> bool {
    matches!(tile_type, Surface::Wall)
}
