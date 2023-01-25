use super::{Confusion, MyTurn, RunState};
use specs::prelude::*;

pub struct TurnStatusSystem {}

impl<'a> System<'a> for TurnStatusSystem {
    type SystemData = (
        WriteStorage<'a, MyTurn>,
        WriteStorage<'a, Confusion>,
        Entities<'a>,
        ReadExpect<'a, RunState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut turns, mut confusion, entities, runstate) = data;

        if *runstate != RunState::Ticking {
            return;
        }

        let mut not_my_turn = Vec::<Entity>::new();
        let mut not_confused = Vec::<Entity>::new();

        for (entity, _, confused) in (&entities, &mut turns, &mut confusion).join() {
            confused.duration -= 1;

            if confused.duration < 1 {
                not_confused.push(entity);
            } else {
                not_my_turn.push(entity);
            }
        }

        for e in not_my_turn {
            turns.remove(e);
        }

        for e in not_confused {
            confusion.remove(e);
        }
    }
}
