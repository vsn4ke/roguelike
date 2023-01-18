use bracket_lib::random::RandomNumberGenerator;

use crate::map::Tile;

use super::Surface;
pub struct DoorPlacement {}

impl DoorPlacement {
    #[allow(dead_code)]
    pub fn new() -> Box<DoorPlacement> {
        Box::new(DoorPlacement {})
    }
}

impl super::MetaMapBuilder for DoorPlacement {
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        if let Some(c) = &data.corridors {
            find_doors_in_corridors(data, c.clone());
        } else {
            find_doors_in_caves(data, data.map.tiles.clone());
        }
    }
}

fn is_entity(entity_list: &[(usize, String)], idx: usize) -> bool {
    for entity in entity_list.iter() {
        if entity.0 == idx {
            return true;
        }
    }

    false
}

fn is_door_possible(map: &super::Map, idx: usize) -> bool {
    let w = map.width as usize;
    let h = map.height as usize;
    let x = idx % w;
    let y = idx / w;

    if x > 1
        && x < w - 2
        && y > 1
        && y < h - 2
        && ((map.tiles[idx].surface == Surface::Floor
            && map.tiles[idx - 1].surface == Surface::Floor
            && map.tiles[idx + 1].surface == Surface::Floor
            && map.tiles[idx - w].surface == Surface::Wall
            && map.tiles[idx + w].surface == Surface::Wall)
            || (map.tiles[idx].surface == Surface::Floor
                && map.tiles[idx - 1].surface == Surface::Wall
                && map.tiles[idx + 1].surface == Surface::Wall
                && map.tiles[idx - w].surface == Surface::Floor
                && map.tiles[idx + w].surface == Surface::Floor))
    {
        return true;
    }

    false
}

fn find_doors_in_corridors(data: &mut super::BuilderMap, corridors: Vec<Vec<usize>>) {
    let mut rng = RandomNumberGenerator::new();
    for corridor in corridors.iter() {
        if corridor.len() < 2 {
            continue;
        }

        for tile in corridor.iter() {
            if !is_door_possible(&data.map, *tile)
                || rng.range(0, 10) > 0
                || is_entity(&data.spawn_list, *tile)
            {
                continue;
            }

            data.spawn_list.push((*tile, "Door".to_string()));
            break;
        }
    }
}

fn find_doors_in_caves(data: &mut super::BuilderMap, tiles: Vec<Tile>) {
    let mut rng = RandomNumberGenerator::new();
    for (idx, tile) in tiles.iter().enumerate() {
        if tile.surface != Surface::Floor
            || !is_door_possible(&data.map, idx)
            || rng.range(0, 10) > 0
            || is_entity(&data.spawn_list, idx)
        {
            continue;
        }
        data.spawn_list.push((idx, "Door".to_string()));
    }
}
