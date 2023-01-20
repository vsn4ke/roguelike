use super::{
    super::colors::*, particle::ParticleBuilder, Attributes, EquipmentSlot, Equipped, Log,
    MeleeWeapon, Name, NaturalProperty, Pools, Position, Skills, SufferDamage, WantsToMelee,
    Wearable,
};
use bracket_lib::{random::RandomNumberGenerator, terminal::to_cp437};
use specs::prelude::*;

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantsToMelee>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Pools>,
        WriteStorage<'a, SufferDamage>,
        WriteExpect<'a, ParticleBuilder>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Attributes>,
        ReadStorage<'a, Skills>,
        ReadStorage<'a, Equipped>,
        ReadStorage<'a, MeleeWeapon>,
        ReadStorage<'a, Wearable>,
        ReadStorage<'a, NaturalProperty>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut wants_melee,
            sources,
            pools,
            mut inflict_damage,
            mut particle_builder,
            positions,
            attributes,
            skills,
            equipped_items,
            melee_weapons,
            wearables,
            natural_properties,
        ) = data;

        for (entity, wants_melee, source, source_attributes, source_skills, source_pools) in (
            &entities,
            &wants_melee,
            &sources,
            &attributes,
            &skills,
            &pools,
        )
            .join()
        {
            if let Some(target_pools) = pools.get(wants_melee.target) {
                if target_pools.hit_points.current <= 0 {
                    continue;
                }
            } else {
                continue;
            }

            if source_pools.hit_points.current <= 0 {
                continue;
            }

            let target = sources.get(wants_melee.target).unwrap();
            let target_attributes = attributes.get(wants_melee.target).unwrap();
            let target_skills = skills.get(wants_melee.target).unwrap();
            let mut rng = RandomNumberGenerator::new();
            let natural_roll = rng.roll_dice(1, 20);

            let mut attack = MeleeWeapon::base();
            if let Some(np) = natural_properties.get(entity) {
                if let Some(idx) = rng.random_slice_index(&np.attacks) {
                    attack.hit_bonus = np.attacks[idx].hit_bonus;
                    attack.damage_bonus = np.attacks[idx].damage_bonus;
                    attack.damage_die_type = np.attacks[idx].damage_die_type;
                    attack.damage_n_dice = np.attacks[idx].damage_n_dice;
                }
            }

            for (wielded, melee) in (&equipped_items, &melee_weapons).join() {
                if wielded.owner == entity && wielded.slot == EquipmentSlot::Melee {
                    attack = *melee;
                }
            }

            let hit_bonus_from_attribute = source_attributes.might.bonus();
            let hit_bonus_from_weapon = attack.hit_bonus;
            let hit_bonus_from_skill = source_skills.melee;

            let hit_roll = natural_roll
                + hit_bonus_from_attribute
                + hit_bonus_from_skill
                + hit_bonus_from_weapon;

            let base_armor_class = if let Some(np) = natural_properties.get(wants_melee.target) {
                np.armor_class.unwrap_or(10)
            } else {
                10
            };

            let armor_bonus_from_quickness = target_attributes.quickness.bonus();
            let armor_bonus_from_skill = target_skills.defense;
            let mut armor_bonus_from_item = 0;

            for (wielded, armor) in (&equipped_items, &wearables).join() {
                if wielded.owner == wants_melee.target {
                    armor_bonus_from_item += armor.armor_class;
                }
            }

            let armor_class_total = base_armor_class
                + armor_bonus_from_item
                + armor_bonus_from_quickness
                + armor_bonus_from_skill;

            if natural_roll > 1 && (natural_roll == 20 || hit_roll >= armor_class_total) {
                let base_damage = rng.roll_dice(attack.damage_n_dice, attack.damage_die_type);
                let bonus_damage_from_attribute = source_attributes.might.bonus();
                let bonus_damage_from_skill = source_skills.melee;
                let bonus_damage_from_weapon = attack.damage_bonus;

                let damage_total = i32::max(
                    0,
                    base_damage
                        + bonus_damage_from_attribute
                        + bonus_damage_from_skill
                        + bonus_damage_from_weapon,
                );
                SufferDamage::new_damage(&mut inflict_damage, wants_melee.target, damage_total);
                Log::new()
                    .append("(Roll")
                    .roll(&format!("{:02}", hit_roll))
                    .append("vs")
                    .roll(&format!("{:02}", armor_class_total))
                    .append("AC|")
                    .npc(&source.name)
                    .append("hits")
                    .npc(&target.name)
                    .append(", for")
                    .bad(&damage_total)
                    .append("hp")
                    .build();
                if let Some(pos) = positions.get(wants_melee.target) {
                    particle_builder.request(
                        pos.x,
                        pos.y,
                        c(YELLOW1),
                        c(BLACK),
                        to_cp437('‼'),
                        200.0,
                    );
                }
            } else if natural_roll == 1 {
                //fumble
                Log::new()
                    .append("(Roll")
                    .bad(&format!("{:02}", natural_roll))
                    .append("vs")
                    .roll(&format!("{:02}", armor_class_total))
                    .append("AC|")
                    .npc(&source.name)
                    .append("attacks")
                    .npc(&target.name)
                    .append(".")
                    .bad(&"Fumble!")
                    .build();
                if let Some(pos) = positions.get(wants_melee.target) {
                    particle_builder.request(
                        pos.x,
                        pos.y,
                        c(BLUE5),
                        c(BLACK),
                        to_cp437('‼'),
                        200.0,
                    );
                }
            } else {
                Log::new()
                    .append("(Roll")
                    .roll(&format!("{:02}", hit_roll))
                    .append("vs")
                    .roll(&format!("{:02}", armor_class_total))
                    .append("AC|")
                    .npc(&source.name)
                    .append("attacks")
                    .npc(&target.name)
                    .append(", but misses.")
                    .build();
                if let Some(pos) = positions.get(wants_melee.target) {
                    particle_builder.request(
                        pos.x,
                        pos.y,
                        c(SHALLOWWATERS5),
                        c(BLACK),
                        to_cp437('‼'),
                        200.0,
                    );
                }
            }
        }

        wants_melee.clear();
    }
}
