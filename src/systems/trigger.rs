use super::{
    super::colors::*, particle::ParticleBuilder, EntityMoved, EntryTrigger, Hidden, InflictsDamage,
    Log, Map, Name, Position, SingleActivation, SufferDamage,
};
use bracket_lib::terminal::to_cp437;
use specs::prelude::*;

pub struct TriggerSystem {}

impl<'a> System<'a> for TriggerSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Map>,
        WriteStorage<'a, EntityMoved>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, EntryTrigger>,
        WriteStorage<'a, Hidden>,
        ReadStorage<'a, Name>,
        Entities<'a>,
        ReadStorage<'a, InflictsDamage>,
        WriteStorage<'a, SufferDamage>,
        WriteExpect<'a, ParticleBuilder>,
        ReadStorage<'a, SingleActivation>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            map,
            mut entity_moved,
            position,
            entry_trigger,
            mut hidden,
            names,
            entities,
            inflicts_damage,
            mut inflict_damage,
            mut particle_builder,
            single_activation,
        ) = data;

        let mut remove_entities = Vec::<Entity>::new();
        for (entity, _, pos) in (&entities, &entity_moved, &position).join() {
            let idx = map.coord_to_index(pos.x, pos.y);
            for entity_id in map.tiles[idx].content.iter() {
                if entity == *entity_id || entry_trigger.get(*entity_id).is_none() {
                    continue;
                }

                if let Some(e) = names.get(*entity_id) {
                    Log::new().item(&e.name).append("triggers!").build();
                }

                hidden.remove(*entity_id);

                if let Some(damage) = inflicts_damage.get(*entity_id) {
                    particle_builder.request(pos.x, pos.y, c(RED3), c(BLACK), to_cp437('‼'), 200.0);
                    SufferDamage::new_damage(&mut inflict_damage, entity, damage.damage, false);
                }

                if single_activation.get(*entity_id).is_some() {
                    remove_entities.push(*entity_id);
                }
            }
        }

        for trap in remove_entities.iter() {
            entities.delete(*trap).expect("Unable to delete trap");
        }

        entity_moved.clear();
    }
}
