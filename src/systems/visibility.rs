use super::{BlocksVisibility, Hidden, Log, Map, Name, Player, Position, RandomGen, Viewshed};
use bracket_lib::prelude::{field_of_view, Algorithm2D};
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Hidden>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, BlocksVisibility>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player, mut hidden, names, blocks_visibility) =
            data;

        let mut rng = RandomGen::default();

        for i in 0..map.tiles.len() {
            map.tiles[i].block_visibility = false;
        }

        for (pos, _) in (&pos, &blocks_visibility).join() {
            let idx = map.coord_to_index(pos.x, pos.y);
            map.tiles[idx].block_visibility = true;
        }

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            if !viewshed.dirty {
                continue;
            }

            viewshed.dirty = false;
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(pos.into_point(), viewshed.range, &*map);
            viewshed.visible_tiles.retain(|p| map.in_bounds(*p));

            if player.get(ent).is_none() {
                continue;
            }

            for t in map.tiles.iter_mut() {
                t.visible = false;
            }
            for vis in viewshed.visible_tiles.iter() {
                let idx = map.point2d_to_index(*vis);
                map.tiles[idx].revealed = true;
                map.tiles[idx].visible = true;

                for e in map.tiles[idx].content.iter() {
                    if hidden.get(*e).is_none() || rng.range(1, 24) > 1 {
                        continue;
                    }

                    if let Some(e) = names.get(*e) {
                        Log::new().append("You spotted a").item(&e.name).build();
                    }

                    hidden.remove(*e);
                }
            }
        }
    }
}
