use super::unit::{Skill, Skills};

pub fn bonus_from_attribute(a: i32) -> i32 {
    (a - 10) / 2
}

pub fn player_hp(fitness: i32, level: i32) -> i32 {
    10 + (10 + bonus_from_attribute(fitness)) * level
}

pub fn npc_hp(fitness: i32, level: i32) -> i32 {
    1 + i32::max(1, 8 + bonus_from_attribute(fitness)) * level
}

pub fn total_mana(intelligence: i32, level: i32) -> i32 {
    i32::max(1, 4 + bonus_from_attribute(intelligence)) * level
}

pub fn skill_bonus(skill: Skill, skills: &Skills) -> i32 {
    if skills.skills.contains_key(&skill) {
        skills.skills[&skill]
    } else {
        -4
    }
}
