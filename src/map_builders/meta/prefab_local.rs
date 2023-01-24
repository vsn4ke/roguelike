use bracket_lib::{prelude::Algorithm2D, terminal::Point};

use super::{BuilderMap, Description, RandomGen, Surface};
use std::collections::HashSet;

pub struct PrefabLocal {
    pub template: &'static str,
    pub width: usize,
    pub height: usize,
    pub first_depth: i32,
    pub last_depth: i32,
}

#[allow(dead_code)]
pub struct PrefabLocalBuilder {
    list: Vec<(PrefabLocal, Vec<Description>)>,
    number_max_of_prefab: usize,
}

impl super::MetaMapBuilder for PrefabLocalBuilder {
    fn build_map(&mut self, data: &mut BuilderMap) {
        let mut rng = RandomGen::default();

        if self.list.is_empty() || self.number_max_of_prefab == 0 {
            return;
        }

        let mut used_tiles: HashSet<usize> = HashSet::new();

        for _ in 0..self.number_max_of_prefab {
            let vault_idx = rng.range(0, self.list.len());
            let vault = &self.list[vault_idx].0;
            let mut vault_positions: Vec<Point> = Vec::new();

            for idx in 0..data.map.tiles.len() {
                let pt = data.map.index_to_point2d(idx);
                if pt.x > 1
                    && (pt.x + vault.width as i32) < data.map.width - 2
                    && pt.y > 1
                    && (pt.y + vault.height as i32) < data.map.height - 2
                {
                    let mut possible = true;
                    for dy in 0..vault.height as i32 {
                        for dx in 0..vault.width as i32 {
                            let idx = data.map.coord_to_index(pt.x + dx, pt.y + dy);
                            if data.map.tiles[idx].surface != Surface::Floor
                                || used_tiles.contains(&idx)
                            {
                                possible = false;
                            }
                        }
                    }

                    if possible {
                        vault_positions.push(pt);
                        break;
                    }
                }
            }

            if vault_positions.is_empty() {
                continue;
            }

            let pos = &vault_positions[rng.range(0, vault_positions.len())];

            data.spawn_list.retain(|e| {
                let idx = e.0 as i32;
                let x = idx % data.map.width;
                let y = idx / data.map.height;
                x < pos.x
                    || x > pos.x + vault.width as i32
                    || y < pos.y
                    || y > pos.y + vault.height as i32
            });

            let char_vec = char_to_vec(vault.template);
            let mut i = 0;
            for ty in 0..vault.height {
                for tx in 0..vault.width {
                    let idx = data
                        .map
                        .coord_to_index(tx as i32 + pos.x, ty as i32 + pos.y);

                    char_to_map(char_vec[i], idx, data, &self.list[vault_idx].1);
                    used_tiles.insert(idx);
                    i += 1;
                }
            }
            data.take_snapshot();
        }
    }
}

impl PrefabLocalBuilder {}

pub fn char_to_vec(template: &str) -> Vec<char> {
    let mut char_vec: Vec<char> = template
        .chars()
        .filter(|a| *a != '\r' && *a != '\n')
        .collect();
    for c in char_vec.iter_mut() {
        if *c as u8 == 160u8 {
            *c = ' ';
        }
    }

    char_vec
}

pub fn char_to_map(ch: char, idx: usize, data: &mut BuilderMap, description: &[Description]) {
    for d in description.iter() {
        if d.char == ch {
            data.map.tiles[idx].surface = d.surface;
            if !d.name.is_empty() {
                data.spawn_list.push((idx, d.name.clone()));
            }

            return;
        }
    }

    println!("Unknown glyph loading map: {}", (ch as u8) as char);
}
