pub mod items;
pub mod loot;
pub mod mobs;
pub mod props;
pub mod rawmaster;
pub mod spawn_table;

use bracket_terminal::{embedded_resource, link_resource, EMBED};
use lazy_static::lazy_static;
pub use rawmaster::*;
use serde::Deserialize;
use serde_json;
use std::sync::Mutex;

use super::{
    colors::c, effect::*, gamesystem::*, item::*, props::*, spawner::random_table::RandomTable,
    unit::*, BlocksTile, BlocksVisibility, Entity, Hidden, Name, Position, Renderable,
};

embedded_resource!(RAW_FILE, "../../raws/spawns.json");

lazy_static! {
    pub static ref RAWS: Mutex<RawMaster> = Mutex::new(RawMaster::empty());
}

pub fn load_raws() {
    link_resource!(RAW_FILE, "../../raws/spawns.json");

    let raw_data = EMBED
        .lock()
        .get_resource("../../raws/spawns.json".to_string())
        .unwrap();

    let raw_string =
        std::str::from_utf8(raw_data).expect("Unable to convert to a valid UTF-8 string");

    let decoder: Raws = serde_json::from_str(raw_string).expect("Unable to parse JSON");

    RAWS.lock().unwrap().load(decoder);
}

#[derive(Deserialize)]
pub struct Raws {
    pub items: Vec<items::ItemRaw>,
    pub mobs: Vec<mobs::MobRaw>,
    pub props: Vec<props::PropRaw>,
    pub spawn_tables: Vec<spawn_table::SpawnTableEntry>,
    pub loot_tables: Vec<loot::LootTableRaw>,
}

#[derive(Deserialize)]
pub struct RenderableRaw {
    pub glyph: String,
    pub fg: String,
    pub bg: String,
    pub order: i32,
}
