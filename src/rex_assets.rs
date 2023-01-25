use bracket_lib::terminal::XpFile;
use std::{fs::File, io::BufReader};

pub struct RexAssets {
    pub menu: XpFile,
}

impl RexAssets {
    #[allow(clippy::new_without_default)]
    pub fn new() -> RexAssets {
        let file = File::open("resources/menu.xp").expect("File path not valid");
        let mut reader = BufReader::new(file);
        RexAssets {
            menu: XpFile::read(&mut reader).unwrap(),
        }
    }
}
