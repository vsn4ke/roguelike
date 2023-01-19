use super::{
    super::colors::c,
    bonus_from_attribute, npc_hp,
    rawmaster::{get_renderable_component, spawn_position},
    spawn_named_entity, total_mana, Attribute, Attributes, BlocksTile, Entity, Faction, Initiative,
    LightSource, LootTable, Movement, MovementMode, Name, NaturalAttack, NaturalProperty, Pool,
    Pools, Quips, RawMaster, RenderableRaw, Skill, Skills, SpawnType, Viewshed,
};
use bracket_lib::random::parse_dice_string;
use serde::Deserialize;
use specs::prelude::*;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct MobRaw {
    pub name: String,
    pub renderable: Option<RenderableRaw>,
    pub blocks_tile: bool,
    pub vision_range: i32,
    pub movement: String,
    pub quips: Option<Vec<String>>,
    pub attributes: AttributesRaw,
    pub skills: Option<HashMap<String, i32>>,
    pub level: Option<i32>,
    pub mana: Option<i32>,
    pub hp: Option<i32>,
    pub equipped: Option<Vec<String>>,
    pub natural: Option<MobNaturalRaw>,
    pub loot_table: Option<String>,
    pub light: Option<LightRaw>,
    pub faction: Option<String>,
}

#[derive(Deserialize)]
pub struct AttributesRaw {
    pub might: Option<i32>,
    pub fitness: Option<i32>,
    pub quickness: Option<i32>,
    pub intelligence: Option<i32>,
}

#[derive(Deserialize)]
pub struct MobNaturalRaw {
    pub armor_class: Option<i32>,
    pub attacks: Option<Vec<NaturalAttackRaw>>,
}

#[derive(Deserialize)]
pub struct NaturalAttackRaw {
    pub name: String,
    pub hit_bonus: i32,
    pub damage: String,
}

#[derive(Deserialize)]
pub struct LightRaw {
    pub color: String,
    pub range: i32,
}

pub fn spawn_named_mob(
    raws: &RawMaster,
    ecs: &mut World,
    key: &str,
    pos: SpawnType,
) -> Option<Entity> {
    if !raws.mob_index.contains_key(key) {
        return None;
    }

    let mob_template = &raws.raws.mobs[raws.mob_index[key]];
    let mut eb = ecs.create_entity();
    eb = spawn_position(pos, eb, key, raws);

    if let Some(renderable) = &mob_template.renderable {
        eb = eb.with(get_renderable_component(renderable));
    }

    eb = eb.with(Name {
        name: mob_template.name.clone(),
    });

    if mob_template.blocks_tile {
        eb = eb.with(BlocksTile {});
    }

    eb = eb.with(Viewshed {
        visible_tiles: Vec::new(),
        range: mob_template.vision_range,
        dirty: true,
    });

    eb = eb.with(MovementMode {
        mode: match mob_template.movement.as_ref() {
            "random" => Movement::Random,
            _ => Movement::Static,
        },
    });

    if let Some(quips) = &mob_template.quips {
        eb = eb.with(Quips {
            available: quips.clone(),
        })
    }

    let mut attr = Attributes {
        might: Attribute {
            base: 11,
            modifiers: 0,
            bonus: 0,
        },
        fitness: Attribute {
            base: 11,
            modifiers: 0,
            bonus: 0,
        },
        quickness: Attribute {
            base: 11,
            modifiers: 0,
            bonus: 0,
        },
        intelligence: Attribute {
            base: 11,
            modifiers: 0,
            bonus: 0,
        },
    };

    if let Some(might) = mob_template.attributes.might {
        attr.might = Attribute {
            base: might,
            modifiers: 0,
            bonus: bonus_from_attribute(might),
        }
    }

    if let Some(fitness) = mob_template.attributes.fitness {
        attr.fitness = Attribute {
            base: fitness,
            modifiers: 0,
            bonus: bonus_from_attribute(fitness),
        }
    }

    if let Some(quickness) = mob_template.attributes.quickness {
        attr.quickness = Attribute {
            base: quickness,
            modifiers: 0,
            bonus: bonus_from_attribute(quickness),
        }
    }

    if let Some(intelligence) = mob_template.attributes.intelligence {
        attr.intelligence = Attribute {
            base: intelligence,
            modifiers: 0,
            bonus: bonus_from_attribute(intelligence),
        }
    }

    eb = eb.with(attr);

    let mob_level = mob_template.level.unwrap_or(1);
    let mob_hp = npc_hp(attr.fitness.base, mob_level);
    let mob_mana = total_mana(attr.intelligence.base, mob_level);

    let pools = Pools {
        level: mob_level,
        xp: 0,
        hit_points: Pool {
            max: mob_hp,
            current: mob_hp,
        },
        mana: Pool {
            max: mob_mana,
            current: mob_mana,
        },
    };

    eb = eb.with(pools);

    let mut skills = Skills {
        skills: HashMap::new(),
    };
    skills.skills.insert(Skill::Melee, 1);
    skills.skills.insert(Skill::Defense, 1);
    skills.skills.insert(Skill::Magic, 1);

    if let Some(mobskills) = &mob_template.skills {
        for s in mobskills.iter() {
            match s.0.as_str() {
                "Melee" => {
                    skills.skills.insert(Skill::Melee, *s.1);
                }
                "Magic" => {
                    skills.skills.insert(Skill::Melee, *s.1);
                }
                "Defense" => {
                    skills.skills.insert(Skill::Melee, *s.1);
                }
                _ => println!("Unknown skill : {}", s.0),
            }
        }
    }

    eb = eb.with(skills);

    if let Some(natural) = &mob_template.natural {
        let mut natural_property = NaturalProperty {
            armor_class: natural.armor_class,
            attacks: Vec::new(),
        };

        if let Some(attacks) = &natural.attacks {
            for attack in attacks.iter() {
                let dice = parse_dice_string(&attack.damage).unwrap();
                natural_property.attacks.push(NaturalAttack {
                    name: attack.name.clone(),
                    hit_bonus: attack.hit_bonus,
                    damage_bonus: dice.bonus,
                    damage_die_type: dice.die_type,
                    damage_n_dice: dice.n_dice,
                });
            }
        }
        eb = eb.with(natural_property);
    }

    if let Some(loot) = &mob_template.loot_table {
        eb = eb.with(LootTable {
            table: loot.clone(),
        });
    }

    if let Some(light) = &mob_template.light {
        eb = eb.with(LightSource {
            range: light.range,
            color: c(&light.color),
        });
    }

    eb = eb.with(Initiative { current: 2 });

    eb = eb.with(Faction {
        name: mob_template
            .faction
            .clone()
            .unwrap_or("Mindless".to_string()),
    });

    //Mob Equippement
    let mob = eb.build();

    if let Some(wielding) = &mob_template.equipped {
        for tag in wielding.iter() {
            spawn_named_entity(raws, ecs, tag, SpawnType::Equipped { by: mob });
        }
    }

    Some(mob)
}
