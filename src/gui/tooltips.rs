use super::super::{
    camera::get_screen_bounds, colors::*, unit::Attributes, Hidden, Map, Name, Position,
};
use bracket_lib::prelude::Algorithm2D;
use bracket_lib::terminal::{to_cp437, BTerm, Point};
use specs::prelude::*;

struct Tooltip {
    lines: Vec<String>,
}

impl Tooltip {
    fn new() -> Tooltip {
        Tooltip { lines: Vec::new() }
    }

    fn add<S: ToString>(&mut self, line: S) {
        self.lines.push(line.to_string());
    }

    fn width(&self) -> i32 {
        let mut max = 0;
        for s in self.lines.iter() {
            if s.len() > max {
                max = s.len();
            }
        }

        max as i32 + 2
    }

    fn height(&self) -> i32 {
        self.lines.len() as i32 + 2
    }

    fn render(&self, ctx: &mut BTerm, x: i32, y: i32) {
        ctx.draw_box(
            x,
            y,
            self.width() - 1,
            self.height() - 1,
            c(GRAY5),
            c(BLACK),
        );
        for (i, s) in self.lines.iter().enumerate() {
            let col = if i == 0 { c(WHITE) } else { c(GRAY5) };
            ctx.print_color(x + 1, y + 1 + i as i32, col, c(BLACK), s);
        }
    }
}

pub fn draw_tooltips(ecs: &World, ctx: &mut BTerm) {
    let player_pos = ecs.fetch::<Point>();
    let map = ecs.fetch::<Map>();
    let names = ecs.read_storage::<Name>();
    let positions = ecs.read_storage::<Position>();
    let hidden = ecs.read_storage::<Hidden>();
    let attributes = ecs.read_storage::<Attributes>();
    let entities = ecs.entities();
    let (min_x, _, min_y, _) = get_screen_bounds(*player_pos);

    let mouse_pt = ctx.mouse_point();
    let mut mouse_pos = mouse_pt;
    mouse_pos.x += min_x;
    mouse_pos.y += min_y;

    if !map.in_bounds(mouse_pos) || !map.tiles[map.point2d_to_index(mouse_pos)].visible {
        return;
    }

    let mut tip_boxes = Vec::<Tooltip>::new();
    for (entity, source, position, _) in (&entities, &names, &positions, !&hidden).join() {
        if position.x == mouse_pos.x && position.y == mouse_pos.y {
            let mut tip = Tooltip::new();
            tip.add(&source.name);

            if let Some(attribute) = attributes.get(entity) {
                let mut s = "".to_string();
                if attribute.might.bonus() < 0 {
                    s += "Weak. "
                }
                if attribute.might.bonus() > 0 {
                    s += "Strong. "
                }
                if attribute.quickness.bonus() < 0 {
                    s += "Clumsy. "
                }
                if attribute.quickness.bonus() > 0 {
                    s += "Agile. "
                }
                if attribute.fitness.bonus() < 0 {
                    s += "Unhealthy. "
                }
                if attribute.fitness.bonus() > 0 {
                    s += "Healthy. "
                }
                if attribute.intelligence.bonus() < 0 {
                    s += "Dumb. "
                }
                if attribute.intelligence.bonus() > 0 {
                    s += "Smart. "
                }

                if s.is_empty() {
                    s = "Quite Average".to_string();
                }
                tip.add(s);

                tip.add(format!("Level: {}", attribute.level));
            }

            tip_boxes.push(tip);
        }
    }

    if tip_boxes.is_empty() {
        return;
    }

    let fg = c(WHITE);
    let bg = c(GRAY4);

    let (arrow_x, arrow) = if mouse_pt.x > 40 {
        (mouse_pt.x - 1, to_cp437('→'))
    } else {
        (mouse_pt.x + 1, to_cp437('←'))
    };

    ctx.set(arrow_x, mouse_pt.y, fg, bg, arrow);

    let mut height = 0;
    for tip in tip_boxes.iter() {
        height += tip.height();
    }

    let mut y = mouse_pt.y - height / 2;
    while y + height / 2 > 50 {
        y -= 1;
    }

    for tip in tip_boxes.iter() {
        let x = if mouse_pt.x < 40 {
            mouse_pt.x - 1 - tip.width()
        } else {
            mouse_pt.x + 1 + tip.width()
        };
        tip.render(ctx, x, y);
        y += tip.height();
    }
}
