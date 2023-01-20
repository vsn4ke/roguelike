use super::super::{Attributes, Initiative, MyTurn, Position, RunState};
use bracket_lib::{
    random::RandomNumberGenerator,
    terminal::{DistanceAlg, Point},
};
use specs::prelude::*;

pub struct InitiativeSystem {}

impl<'a> System<'a> for InitiativeSystem {
    type SystemData = (
        WriteStorage<'a, Initiative>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, MyTurn>,
        Entities<'a>,
        ReadStorage<'a, Attributes>,
        WriteExpect<'a, RunState>,
        ReadExpect<'a, Entity>,
        ReadExpect<'a, Point>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut initiatives,
            positions,
            mut turns,
            entities,
            attributes,
            mut runstate,
            player,
            player_pos,
        ) = data;
        let mut rng = RandomNumberGenerator::new();

        if *runstate != RunState::Ticking {
            return;
        }
        turns.clear();

        for (entity, initiative, pos) in (&entities, &mut initiatives, &positions).join() {
            initiative.current -= 1;
            if initiative.current >= 1 {
                continue;
            }

            let mut myturn = true;
            turns
                .insert(entity, MyTurn {})
                .expect("Unable to insert turn");
            initiative.current = 6 + rng.range(0, 6);

            if let Some(attr) = attributes.get(entity) {
                initiative.current += attr.initiative_bonus();
            }

            if entity == *player {
                *runstate = RunState::AwaitingInput;
            } else {
                let distance =
                    DistanceAlg::PythagorasSquared.distance2d(*player_pos, pos.into_point());
                if distance > 150.0 {
                    myturn = false;
                }
            }

            if myturn {
                turns
                    .insert(entity, MyTurn {})
                    .expect("Unable to insert turn");
            }
        }
    }
}
