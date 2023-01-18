use bracket_lib::terminal::XpFile;
use bracket_terminal::{embedded_resource, link_resource, EMBED};

embedded_resource!(DUGEON, "../resources/menu.xp");

pub struct RexAssets {
    pub menu: XpFile,
}

impl RexAssets {
    #[allow(clippy::new_without_default)]
    pub fn new() -> RexAssets {
        link_resource!(DUGEON, "../resources/menu.xp");

        RexAssets {
            menu: XpFile::from_resource("../resources/menu.xp").unwrap(),
        }
    }
}
