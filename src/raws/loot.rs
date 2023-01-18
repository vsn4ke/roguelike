use super::{RandomTable, RawMaster};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LootTableRaw {
    pub name: String,
    pub drops: Vec<LootDropRaw>,
}

#[derive(Deserialize)]
pub struct LootDropRaw {
    pub name: String,
    pub weight: i32,
}

pub fn get_loots(raws: &RawMaster, table: &str) -> Option<String> {
    if !raws.loot_index.contains_key(table) {
        return None;
    }

    let mut random_table = RandomTable::new();
    let options = &raws.raws.loot_tables[raws.loot_index[table]];
    for item in options.drops.iter() {
        random_table = random_table.add(item.name.clone(), item.weight);
    }

    Some(random_table.roll())
}
