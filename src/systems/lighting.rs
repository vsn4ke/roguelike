use super::{
    super::colors::{c, BLACK},
    LightSource, Map, Position, Viewshed,
};
use bracket_lib::{prelude::Algorithm2D, terminal::DistanceAlg};
use specs::prelude::*;

pub struct LightingSystem {}

impl<'a> System<'a> for LightingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, LightSource>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, viewshed, positions, lighting) = data;

        if map.outdoors {
            return;
        }

        for tile in map.tiles.iter_mut() {
            tile.light = c(BLACK);
        }

        for (viewshed, pos, light) in (&viewshed, &positions, &lighting).join() {
            let light_point = pos.into_point();
            let range_f = light.range as f32;
            for t in viewshed.visible_tiles.iter() {
                if !map.in_bounds(*t) {
                    continue;
                }
                let idx = map.point2d_to_index(*t);
                let distance = DistanceAlg::PythagorasSquared.distance2d(light_point, *t);
                let intensity = 1.0 - distance / (range_f * range_f);

                map.tiles[idx].light = map.tiles[idx].light + light.color * intensity;
            }
        }
    }
}
