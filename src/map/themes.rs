use bracket_lib::terminal::{to_cp437, RGB};
use bracket_terminal::FontCharType;

use super::{super::colors::*, tiles::Surface, Map};

pub fn get_tile_glyph(idx: usize, map: &Map) -> (FontCharType, RGB, RGB) {
    let (glyph, mut fg, theme_bg) = match map.depth {
        1 => get_forest_glyph(idx, map),
        _ => get_default_glyph(idx, map),
    };

    let bg = if map.tiles[idx].bloodstains && map.tiles[idx].visible {
        RGB::from_f32(0.75, 0., 0.)
    } else {
        theme_bg
    };

    if !map.tiles[idx].visible {
        fg = fg.to_greyscale();
    }

    (glyph, fg, bg)
}

pub fn get_default_glyph(idx: usize, map: &Map) -> (FontCharType, RGB, RGB) {
    let (glyph, fg) = match map.tiles[idx].surface {
        Surface::Floor => ('.', c(GRAY5)),
        Surface::Wall => (wall_glyph(map, idx), c(BROWN3)),
        Surface::DownStairs => ('>', c(SHALLOWWATERS1)),
        Surface::UpStairs => ('<', c(SHALLOWWATERS1)),
        Surface::Bridge => ('▒', c(BROWN1)),
        Surface::DeepWater => ('≈', c(DEEPSEA4)),
        Surface::Grass => ('"', c(GREEN5)),
        Surface::Road => ('░', c(GRAY5)),
        Surface::ShallowWater => ('~', c(SHALLOWWATERS5)),
        Surface::WoodFloor => ('.', c(BROWN5)),
        Surface::Gravel => ('~', c(GRAY2)),
        Surface::Path => ('░', c(GRAY3)),
    };

    (to_cp437(glyph), fg, c(BLACK))
}

pub fn get_forest_glyph(idx: usize, map: &Map) -> (FontCharType, RGB, RGB) {
    let (glyph, fg) = match map.tiles[idx].surface {
        Surface::Wall => ('♣', c(GREEN3)),
        Surface::DownStairs => ('>', c(SHALLOWWATERS1)),
        Surface::UpStairs => ('<', c(SHALLOWWATERS1)),
        Surface::Road => ('≡', c(GRAY3)),
        Surface::Bridge => ('▒', c(BROWN1)),
        Surface::DeepWater => ('≈', c(DEEPSEA4)),
        Surface::ShallowWater => ('~', c(SHALLOWWATERS5)),
        Surface::Gravel => ('~', c(GRAY2)),
        _ => ('"', c(GREEN5)),
    };

    (to_cp437(glyph), fg, c(BLACK))
}

fn wall_glyph(map: &Map, idx: usize) -> char {
    if idx < map.width as usize + 1 || idx > map.width as usize * (map.height as usize - 1) - 1 {
        return '#';
    }

    fn is_revealed_and_wall(map: &Map, idx: usize) -> bool {
        map.tiles[idx].surface == Surface::Wall && map.tiles[idx].revealed
    }

    let mut mask: u8 = 0;

    if is_revealed_and_wall(map, idx - map.width as usize) {
        mask += 1;
    }
    if is_revealed_and_wall(map, idx + map.width as usize) {
        mask += 2;
    }
    if is_revealed_and_wall(map, idx - 1) {
        mask += 4;
    }
    if is_revealed_and_wall(map, idx + 1) {
        mask += 8;
    }

    match mask {
        0 => '○',
        1 => '║',
        2 => '║',
        3 => '║',
        4 => '═',
        5 => '╝',
        6 => '╗',
        7 => '╣',
        8 => '═',
        9 => '╚',
        10 => '╔',
        11 => '╠',
        12 => '═',
        13 => '╩',
        14 => '╦',
        15 => '╬',
        _ => '#',
    }
}
