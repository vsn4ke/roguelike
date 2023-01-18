use bracket_lib::{
    prelude::Algorithm2D,
    random::RandomNumberGenerator,
    terminal::{Point, Rect},
};

use super::{BuilderMap, InitialMapBuilder, Surface};

pub struct BspDungeonBuilder {
    rects: Vec<Rect>,
}

impl InitialMapBuilder for BspDungeonBuilder {
    #[allow(dead_code)]
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        let mut rooms: Vec<Rect> = Vec::new();
        self.rects.clear();
        self.rects.push(Rect::with_size(
            2,
            2,
            data.map.width - 5,
            data.map.height - 5,
        ));
        let first_room = self.rects[0];
        self.add_subrects(first_room);

        let mut n_rooms = 0;
        while n_rooms < 240 {
            let rect = self.get_random_rect();
            let candidate = self.get_random_sub_rect(rect);

            if self.is_possible(candidate, data, &rooms) {
                rooms.push(candidate);
                self.add_subrects(rect);
            }
            n_rooms += 1;
        }

        data.rooms = Some(rooms);
    }
}

impl BspDungeonBuilder {
    #[allow(dead_code)]
    pub fn new() -> Box<BspDungeonBuilder> {
        Box::new(BspDungeonBuilder { rects: Vec::new() })
    }

    fn add_subrects(&mut self, rect: Rect) {
        let width = i32::abs(rect.x1 - rect.x2);
        let height = i32::abs(rect.y1 - rect.y2);
        let half_width = i32::max(width / 2, 1);
        let half_height = i32::max(height / 2, 1);

        self.rects
            .push(Rect::with_size(rect.x1, rect.x2, half_width, half_height));
        self.rects.push(Rect::with_size(
            rect.x1,
            rect.y1 + half_height,
            half_width,
            half_height,
        ));
        self.rects.push(Rect::with_size(
            rect.x1 + half_width,
            rect.y1,
            half_width,
            half_height,
        ));
        self.rects.push(Rect::with_size(
            rect.x1 + half_width,
            rect.y1 + half_height,
            half_width,
            half_height,
        ));
    }

    fn get_random_rect(&mut self) -> Rect {
        let mut rng = RandomNumberGenerator::new();
        if self.rects.len() == 1 {
            return self.rects[0];
        }
        let idx = (rng.roll_dice(1, self.rects.len() as i32) - 1) as usize;
        self.rects[idx]
    }

    fn get_random_sub_rect(&mut self, rect: Rect) -> Rect {
        let mut rng = RandomNumberGenerator::new();
        let mut result = rect;
        let rect_width = i32::abs(rect.x1 - rect.x2);
        let rect_height = i32::abs(rect.y1 - rect.y2);

        let w = i32::max(3, rng.roll_dice(1, i32::min(rect_width, 10)) - 1) + 1;
        let h = i32::max(3, rng.roll_dice(1, i32::min(rect_height, 10)) - 1) + 1;

        result.x1 += rng.roll_dice(1, 6) - 1;
        result.y1 += rng.roll_dice(1, 6) - 1;
        result.x2 = result.x1 + w;
        result.y2 = result.y1 + h;

        result
    }

    fn is_possible(&self, rect: Rect, data: &BuilderMap, rooms: &[Rect]) -> bool {
        let mut expanded = rect;
        expanded.x1 -= 2;
        expanded.x2 += 2;
        expanded.y1 -= 2;
        expanded.y1 += 2;

        for r in rooms.iter() {
            if r.intersect(&rect) {
                return false;
            }
        }

        for y in expanded.y1..=expanded.y2 {
            for x in expanded.x1..=expanded.x2 {
                let p = Point::new(x, y);

                if !data.map.in_bounds(p) {
                    return false;
                }

                if data.map.tiles[data.map.point2d_to_index(p)].surface != Surface::Wall {
                    return false;
                }
            }
        }

        true
    }
}
