pub mod factions;
pub mod items;
pub mod loot;
pub mod mobs;
pub mod props;
pub mod rawmaster;
pub mod spawn_table;

use lazy_static::lazy_static;
pub use rawmaster::*;
use serde::Deserialize;
use serde_json::from_reader;
use std::{fs::File, io::BufReader, sync::Mutex};

use super::{
    colors::c,
    effect::*,
    item::*,
    props::*,
    rng::{parse_dice_string, RandomGen},
    spawner::random_table::RandomTable,
    unit::*,
    BlocksTile, BlocksVisibility, Entity, Hidden, Name, Position, Renderable,
};

lazy_static! {
    pub static ref RAWS: Mutex<RawMaster> = Mutex::new(RawMaster::empty());
}

pub fn load_raws() {
    fn open(path: &str) -> BufReader<File> {
        let file = File::open(path).expect("File path not valid");
        BufReader::new(file)
    }
    let e = "Unable to parse JSON";

    let raws = Raws {
        items: from_reader(open("raws/items.json")).expect(e),
        mobs: from_reader(open("raws/mobs.json")).expect(e),
        props: from_reader(open("raws/props.json")).expect(e),
        spawn_tables: from_reader(open("raws/table_spawn.json")).expect(e),
        loot_tables: from_reader(open("raws/table_loot.json")).expect(e),
        faction_tables: from_reader(open("raws/table_faction.json")).expect(e),
    };

    RAWS.lock().unwrap().load(raws);
}

#[derive(Deserialize)]
pub struct Raws {
    pub items: Vec<items::ItemRaw>,
    pub mobs: Vec<mobs::MobRaw>,
    pub props: Vec<props::PropRaw>,
    pub spawn_tables: Vec<spawn_table::SpawnTableEntry>,
    pub loot_tables: Vec<loot::LootTableRaw>,
    pub faction_tables: Vec<factions::FactionInfoRaw>,
}

#[derive(Deserialize, Debug)]
pub struct RenderableRaw {
    pub glyph: String,
    pub fg: String,
    pub bg: String,
    pub order: i32,
}
