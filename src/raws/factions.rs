use super::RawMaster;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
pub enum Reaction {
    Ignore,
    Attack,
    Flee,
}

#[derive(Deserialize)]
pub struct FactionInfoRaw {
    pub name: String,
    pub responses: HashMap<String, String>,
}

pub fn faction_reaction(my: &str, their: &str, raws: &RawMaster) -> Reaction {
    if raws.faction_index.contains_key(my) {
        let my_faction = &raws.faction_index[my];
        if my_faction.contains_key(their) {
            return my_faction[their];
        }
        if my_faction.contains_key("Default") {
            return my_faction["Default"];
        }
    }
    Reaction::Ignore
}
