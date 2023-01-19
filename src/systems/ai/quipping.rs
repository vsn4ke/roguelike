use super::{Log, MyTurn, Name, Quips, Viewshed};
use bracket_lib::{random::RandomNumberGenerator, terminal::Point};
use specs::prelude::*;

pub struct QuipSystem {}

impl<'a> System<'a> for QuipSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'a, Quips>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, MyTurn>,
        ReadExpect<'a, Point>,
        ReadStorage<'a, Viewshed>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut quips, names, turns, player_pos, viewsheds) = data;
        let mut rng = RandomNumberGenerator::new();

        for (quip, source, viewshed, _) in (&mut quips, &names, &viewsheds, &turns).join() {
            if !viewshed.visible_tiles.contains(&player_pos) || rng.range(0, 6) > 0 {
                continue;
            }

            if let Some(idx) = rng.random_slice_index(&quip.available) {
                Log::new()
                    .npc(&source.name)
                    .append("says \"")
                    .item(&quip.available[idx])
                    .append("\"")
                    .build();

                quip.available.remove(idx);
            }
        }
    }
}
