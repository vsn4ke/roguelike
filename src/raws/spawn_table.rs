use super::{RandomTable, RawMaster};
use serde::Deserialize;
#[derive(Deserialize)]
pub struct SpawnTableEntry {
    pub name: String,
    pub weight: i32,
    pub min_depth: i32,
    pub max_depth: i32,
    pub add_map_depth_to_weight: Option<bool>,
}

pub fn get_spawn_table_for_depth(raws: &RawMaster, depth: i32) -> RandomTable {
    let options: Vec<&SpawnTableEntry> = raws
        .raws
        .spawn_tables
        .iter()
        .filter(|a| depth >= a.min_depth && depth <= a.max_depth)
        .collect();

    let mut table = RandomTable::new();
    for o in options.iter() {
        let mut weight = o.weight;
        if o.add_map_depth_to_weight.is_some() {
            weight += depth;
        }
        table = table.add(o.name.clone(), weight);
    }

    table
}
