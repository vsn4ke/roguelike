use super::{
    super::colors::c,
    parse_dice_string,
    rawmaster::{get_renderable_component, spawn_position},
    spawn_named_entity, Attribute, Attributes, BlocksTile, Entity, EquipmentChanged, Faction,
    Initiative, LightSource, LootTable, Movement, MovementMode, Name, NaturalAttack,
    NaturalProperty, Pools, Quips, RandomGen, RawMaster, RenderableRaw, Skills, SpawnType, Vendor,
    Viewshed,
};
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
    pub money: Option<String>,
    pub vendor: Option<Vec<String>>,
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
    let mut rng = RandomGen::default();
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
            "waypoint" => Movement::Waypoint { path: None },
            _ => Movement::Static,
        },
    });

    if let Some(quips) = &mob_template.quips {
        eb = eb.with(Quips {
            available: quips.clone(),
        })
    }

    let mut attr = Attributes::default();

    if let Some(might) = mob_template.attributes.might {
        attr.might = Attribute::new(might);
    }

    if let Some(fitness) = mob_template.attributes.fitness {
        attr.fitness = Attribute::new(fitness);
    }

    if let Some(quickness) = mob_template.attributes.quickness {
        attr.quickness = Attribute::new(quickness);
    }

    if let Some(intelligence) = mob_template.attributes.intelligence {
        attr.intelligence = Attribute::new(intelligence)
    }

    attr.level = mob_template.level.unwrap_or(1);

    eb = eb.with(attr);

    let mut pools = Pools::new_npc(attr);

    if let Some(money) = &mob_template.money {
        pools.money = rng.roll_str(money);
    }

    eb = eb.with(pools);

    let mut skills = Skills::default();

    if let Some(mobskills) = &mob_template.skills {
        for s in mobskills.iter() {
            match s.0.as_str() {
                "Melee" => {
                    skills.melee = *s.1;
                }
                "Magic" => {
                    skills.magic = *s.1;
                }
                "Defense" => {
                    skills.defense = *s.1;
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
                let dice = parse_dice_string(&attack.damage);
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
            .unwrap_or_else(|| "Mindless".to_string()),
    });

    eb = eb.with(EquipmentChanged {});

    if let Some(vendor) = &mob_template.vendor {
        eb = eb.with(Vendor {
            categories: vendor.clone(),
        });
    }

    //Mob Equippement
    let mob = eb.build();

    if let Some(wielding) = &mob_template.equipped {
        for tag in wielding.iter() {
            spawn_named_entity(raws, ecs, tag, SpawnType::Equipped { by: mob });
        }
    }

    Some(mob)
}
