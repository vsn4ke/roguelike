use std::collections::HashSet;

use bracket_lib::{
    prelude::{a_star_search, Algorithm2D},
    random::RandomNumberGenerator,
    terminal::{DistanceAlg, Point, Rect},
};

use crate::map::spatial::is_blocked;

use super::{BuilderChain, BuilderMap, Surface};

#[derive(Debug)]
enum BuildingTag {
    Pub,
    Temple,
    Blacksmith,
    Clothier,
    Alchemist,
    PlayerHouse,
    Hovel,
    Abandoned,
    Unassigned,
}

pub fn town_builder(depth: i32, width: i32, height: i32) -> BuilderChain {
    let mut builder = BuilderChain::new(depth, width, height, "Town of Lost Hope");
    builder.start_with(TownBuilder::new());

    builder
}

pub struct TownBuilder {}

impl super::InitialMapBuilder for TownBuilder {
    #[allow(dead_code)]
    fn build_map(&mut self, data: &mut BuilderMap) {
        let mut rng = RandomNumberGenerator::new();
        let usable_area = get_usable_area(data);
        let gap = rng.range(usable_area.y1 + 6, usable_area.y2 - 6);

        for tile in data.map.tiles.iter_mut() {
            tile.visible = true;
        }

        grass_layer(data);
        water_and_piers(data);

        let mut available_building_tiles = walls(data, gap, 3);
        let mut buildings = buildings(data, &mut available_building_tiles);
        let doors = add_doors(data, &mut buildings);
        add_paths(data, &doors);

        let building_index = sort_buildings(&mut buildings);
        building_factory(data, &buildings, &building_index);
        spawn_townsfolk(data, &mut available_building_tiles);

        data.starting_point = Some(buildings[0].0.center());

        for y in gap - 2..=gap + 2 {
            let idx = data.map.coord_to_index(data.width - 1, y);
            data.map.tiles[idx].surface = Surface::DownStairs;
        }
        data.take_snapshot();
    }
}

impl TownBuilder {
    pub fn new() -> Box<TownBuilder> {
        Box::new(TownBuilder {})
    }
}

fn grass_layer(data: &mut BuilderMap) {
    for tile in data.map.tiles.iter_mut() {
        tile.surface = Surface::Grass;
    }
    data.take_snapshot();
}

fn water_and_piers(data: &mut BuilderMap) {
    let mut rng = RandomNumberGenerator::new();
    let mut n = (rng.range(1, 65536) as f32) / 65535f32;
    let mut water_width: Vec<i32> = Vec::new();
    for y in 0..data.height {
        let n_water = (f32::sin(n) * 10.0) as i32 + 14 + rng.range(1, 7);
        water_width.push(n_water);
        n += 0.1;
        for x in 0..n_water {
            let idx = data.map.coord_to_index(x, y);
            data.map.tiles[idx].surface = Surface::DeepWater;
        }
        for x in n_water..n_water + 3 {
            let idx = data.map.coord_to_index(x, y);
            data.map.tiles[idx].surface = Surface::ShallowWater;
        }
    }
    data.take_snapshot();

    let mut dock_tiles = Vec::new();

    for _ in 0..rng.range(7, 12) {
        let y = rng.range(0, data.height);
        for x in rng.range(3, 9)..water_width[y as usize] + 4 {
            let idx = data.map.coord_to_index(x, y);
            data.map.tiles[idx].surface = Surface::Bridge;
            dock_tiles.push(idx);
        }
    }

    spawn_dockers(data, &mut dock_tiles);

    data.take_snapshot();
}

fn get_usable_area(data: &mut BuilderMap) -> Rect {
    let x = 34;
    let y = 2;
    let w = data.width - 2 - x;
    let h = data.height - 2 - y;
    Rect::with_size(x, y, w, h)
}

fn walls(data: &mut BuilderMap, gap_y: i32, gap_size: i32) -> HashSet<usize> {
    let mut available_building_tiles: HashSet<usize> = HashSet::new();
    let usable_area = get_usable_area(data);
    let offset = 2;

    for y in usable_area.y1..=usable_area.y2 {
        for x in usable_area.x1..=usable_area.x2 {
            let idx = data.map.coord_to_index(x, y);

            if (y == usable_area.y1
                || y == usable_area.y2
                || ((x == usable_area.x1 || x == usable_area.x2)
                    && y > usable_area.y1
                    && y < usable_area.y2))
                && data.map.tiles[idx].surface != Surface::Road
            {
                data.map.tiles[idx].surface = Surface::Wall;
                continue;
            }

            if y > gap_y - gap_size && y < gap_y + gap_size {
                if x == usable_area.x1 + 1 {
                    data.map.tiles[idx - 1].surface = Surface::Road;
                    data.map.tiles[idx - 2].surface = Surface::Road;
                }

                if x == usable_area.x2 - 1 {
                    data.map.tiles[idx + 1].surface = Surface::Road;
                    data.map.tiles[idx + 2].surface = Surface::Road;
                }

                data.map.tiles[idx].surface = Surface::Road;
                continue;
            }

            data.map.tiles[idx].surface = Surface::Gravel;
            if y > usable_area.y1 + offset
                && y < usable_area.y2 - offset
                && x > usable_area.x1 + offset
                && x < usable_area.x2 - offset
            {
                available_building_tiles.insert(idx);
            }
        }
    }

    data.take_snapshot();
    available_building_tiles
}

fn buildings(
    data: &mut BuilderMap,
    available_building_tiles: &mut HashSet<usize>,
) -> Vec<(Rect, Vec<usize>)> {
    let mut rng = RandomNumberGenerator::new();
    let mut buildings: Vec<(Rect, Vec<usize>)> = Vec::new();
    let mut number_of_buildings = 0;
    let mut tries = 400;
    let usable_area = get_usable_area(data);
    while number_of_buildings < 12 && tries > 0 {
        let bx = rng.range(usable_area.x1, usable_area.x2);
        let by = rng.range(usable_area.y1, usable_area.y2);
        let bw = rng.range(5, 12);
        let bh = rng.range(5, 12);
        let b = Rect::with_size(bx, by, bw, bh);

        let mut possible = true;

        for y in b.y1..b.y2 {
            for x in b.x1..b.x2 {
                let idx = data.map.coord_to_index(x, y);
                if !available_building_tiles.contains(&idx) {
                    possible = false;
                }
            }
        }

        if !possible {
            tries -= 1;
            continue;
        }

        number_of_buildings += 1;

        let mut possible_doors = Vec::new();

        for y in b.y1..b.y2 {
            for x in b.x1..b.x2 {
                let idx = data.map.coord_to_index(x, y);

                if y == b.y1 {
                    available_building_tiles.remove(&(idx - data.width as usize));
                }

                if y == b.y2 - 1 {
                    available_building_tiles.remove(&(idx + data.width as usize));
                }

                if x == b.x1 {
                    available_building_tiles.remove(&(idx - 1));
                }

                if x == b.x2 - 1 {
                    available_building_tiles.remove(&(idx + 1));
                }

                available_building_tiles.remove(&idx);

                if y == b.y1
                    || y == b.y2 - 1
                    || ((x == b.x1 || x == b.x2 - 1) && y > b.y1 && y < b.y2 - 1)
                {
                    data.map.tiles[idx].surface = Surface::Wall;

                    if x != b.x1 && x != b.x2 - 1 || y != b.y1 && y != b.y2 - 1 {
                        possible_doors.push(idx);
                    }
                } else {
                    data.map.tiles[idx].surface = Surface::WoodFloor;
                }
            }
        }
        buildings.push((b, possible_doors));
        data.take_snapshot();
    }

    buildings
}

fn add_paths(data: &mut BuilderMap, doors: &[usize]) {
    let mut roads = Vec::new();
    for y in 0..data.height {
        for x in 0..data.width {
            let idx = data.map.coord_to_index(x, y);
            if data.map.tiles[idx].surface == Surface::Road {
                roads.push(idx);
            }
        }
    }

    data.map.populate_blocked();
    for door_idx in doors.iter() {
        let mut nearest_roads: Vec<(usize, f32)> = Vec::new();
        let door_pt = Point::new(
            *door_idx as i32 % data.map.width,
            *door_idx as i32 / data.map.width,
        );
        for r in roads.iter() {
            nearest_roads.push((
                *r,
                DistanceAlg::PythagorasSquared.distance2d(
                    door_pt,
                    Point::new(*r as i32 % data.map.width, *r as i32 / data.map.width),
                ),
            ));
        }
        nearest_roads.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let destination = nearest_roads[0].0;
        let path = a_star_search(*door_idx, destination, &data.map);
        if path.success {
            for step in path.steps.iter() {
                let idx = *step;
                if data.map.tiles[idx].surface != Surface::Road {
                    data.map.tiles[idx].surface = Surface::Path;
                    roads.push(idx);
                }
            }
        }
        data.take_snapshot();
    }
}

fn add_doors(data: &mut BuilderMap, buildings: &mut [(Rect, Vec<usize>)]) -> Vec<usize> {
    let mut rng = RandomNumberGenerator::new();
    let mut doors = Vec::new();

    for building in buildings.iter() {
        let roll = rng.range(0, building.1.len());
        let idx = building.1[roll];

        data.map.tiles[idx].surface = Surface::Floor;
        data.spawn_list.push((idx, "Door".to_string()));
        doors.push(idx);
    }
    data.take_snapshot();
    doors
}

fn sort_buildings(buildings: &mut [(Rect, Vec<usize>)]) -> Vec<(usize, i32, BuildingTag)> {
    let mut size: Vec<(usize, i32, BuildingTag)> = Vec::new();

    buildings.sort_by(|a, b| {
        let bsize = b.0.width() * b.0.height();
        let asize = a.0.width() * a.0.height();
        bsize.cmp(&asize)
    });

    for (i, b) in buildings.iter().enumerate() {
        let bsize = b.0.width() * b.0.height();
        size.push((i, bsize, BuildingTag::Unassigned));
    }

    size[0].2 = BuildingTag::Pub;
    size[1].2 = BuildingTag::Temple;
    size[2].2 = BuildingTag::Blacksmith;
    size[3].2 = BuildingTag::Clothier;
    size[4].2 = BuildingTag::Alchemist;
    size[5].2 = BuildingTag::PlayerHouse;

    for b in size.iter_mut().skip(6) {
        b.2 = BuildingTag::Hovel;
    }

    let last_idx = size.len() - 1;
    size[last_idx].2 = BuildingTag::Abandoned;

    size
}

fn building_factory(
    data: &mut BuilderMap,
    buildings: &[(Rect, Vec<usize>)],
    building_index: &[(usize, i32, BuildingTag)],
) {
    use BuildingTag::*;
    let mut rng = RandomNumberGenerator::new();
    for (i, building) in buildings.iter().enumerate() {
        let mut interior = match &building_index[i].2 {
            Pub => build_pub(),
            Temple => build_temple(),
            Blacksmith => build_blacksmith(),
            Clothier => build_clothier(),
            Alchemist => build_alchemist(),
            PlayerHouse => build_player_house(),
            Hovel => build_hovel(),
            Abandoned => build_abandonned(rng.range(4, 10)),
            Unassigned => Vec::new(),
        };

        let idx = match &building_index[i].2 {
            Pub => place_player(building.0, data),
            _ => 0,
        };

        random_building_spawn(building.0, data, &mut interior, idx);
    }
}

fn random_building_spawn(
    building: Rect,
    data: &mut BuilderMap,
    to_place: &mut Vec<&str>,
    player_idx: usize,
) {
    let mut used_indexes = Vec::new();
    let mut rng = RandomNumberGenerator::new();

    for _ in 0..200 {
        if to_place.is_empty() {
            break;
        }

        let p = Point::new(
            rng.range(building.x1 + 1, building.x2 - 1),
            rng.range(building.y1 + 1, building.y2 - 1),
        );
        let idx = data.map.point2d_to_index(p);

        if idx != player_idx && !used_indexes.contains(&idx) && !is_blocked(idx) {
            used_indexes.push(idx);
            data.spawn_list
                .push((idx, to_place.pop().unwrap().to_string()));
        }
    }
}

fn place_player(building: Rect, data: &mut BuilderMap) -> usize {
    let center = building.center();
    data.starting_point = Some(center);
    data.map.point2d_to_index(center)
}

fn build_pub() -> Vec<&'static str> {
    vec![
        "Table",
        "Chair",
        "Table",
        "Chair",
        "Keg",
        "Patron",
        "Patron",
        "Shady Salesman",
        "Barkeep",
    ]
}

fn build_temple() -> Vec<&'static str> {
    vec![
        "Chair",
        "Chair",
        "Candle",
        "Candle",
        "Parishioner",
        "Parishioner",
        "Priest",
    ]
}

fn build_blacksmith() -> Vec<&'static str> {
    vec![
        "Blacksmith",
        "Anvil",
        "Water Trough",
        "Weapon Rack",
        "Armor Stand",
    ]
}
fn build_clothier() -> Vec<&'static str> {
    vec!["Clothier", "Cabinet", "Table", "Loom", "Hide Rack"]
}
fn build_alchemist() -> Vec<&'static str> {
    vec!["Alchemist", "Chemistry Set", "Dead Thing", "Chair", "Table"]
}
fn build_player_house() -> Vec<&'static str> {
    vec!["Mom", "Bed", "Cabinet", "Chair", "Table"]
}
fn build_hovel() -> Vec<&'static str> {
    vec!["Peasant", "Bed", "Chair", "Table"]
}

fn build_abandonned(n_rats: usize) -> Vec<&'static str> {
    vec!["Rat"; n_rats]
}

fn spawn_dockers(data: &mut BuilderMap, dock_tiles: &mut Vec<usize>) {
    let mut rng = RandomNumberGenerator::new();
    let dockers = ["Dock Worker", "Wannabe Pirate", "Fisher"];
    if dock_tiles.len() < 10 {
        println!("Too few dock tiles.");
        return;
    }

    for _ in 0..10 {
        let idx = dock_tiles.remove(rng.range(0, dock_tiles.len()));
        let name = dockers[rng.range(0, dockers.len())].to_string();
        data.spawn_list.push((idx, name));
    }
}

fn spawn_townsfolk(data: &mut BuilderMap, available_building_tiles: &mut HashSet<usize>) {
    let mut rng = RandomNumberGenerator::new();
    let townsfolk = ["Peasant", "Drunk", "Dock Worker", "Fisher"];

    let area = get_usable_area(data);
    let mut townsfolk_to_place = 10;
    let mut tries = 0;

    while tries < 200 && townsfolk_to_place > 0 {
        let p = Point::new(rng.range(area.x1, area.x2), rng.range(area.y1, area.y2));
        let idx = data.map.point2d_to_index(p);

        if !available_building_tiles.contains(&idx) && !is_blocked(idx) {
            let name = townsfolk[rng.range(0, townsfolk.len())].to_string();
            data.spawn_list.push((idx, name));
            available_building_tiles.remove(&idx);
            townsfolk_to_place -= 1;
        }
        tries += 1;
    }
}
