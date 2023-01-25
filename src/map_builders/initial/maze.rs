use bracket_lib::{prelude::Algorithm2D, terminal::Point};

use super::{BuilderMap, Map, RandomGen, Surface};

const TOP: usize = 0;
const RIGHT: usize = 1;
const BOTTOM: usize = 2;
const LEFT: usize = 3;

#[derive(Copy, Clone)]
struct Cell {
    row: i32,
    column: i32,
    walls: [bool; 4],
    visited: bool,
}

impl Cell {
    fn new(row: i32, column: i32) -> Cell {
        Cell {
            row,
            column,
            walls: [true, true, true, true],
            visited: false,
        }
    }

    fn remove_walls(&mut self, next: &mut Cell) {
        let x = self.column - next.column;
        let y = self.row - next.row;

        if x == 1 {
            self.walls[LEFT] = false;
            next.walls[RIGHT] = false;
        } else if x == -1 {
            self.walls[RIGHT] = false;
            next.walls[LEFT] = false;
        } else if y == 1 {
            self.walls[TOP] = false;
            next.walls[BOTTOM] = false;
        } else if y == -1 {
            self.walls[BOTTOM] = false;
            next.walls[TOP] = false;
        }
    }
}

struct Grid {
    width: i32,
    height: i32,
    cells: Vec<Cell>,
    backtrace: Vec<usize>,
    current: usize,
}

impl Grid {
    fn new(width: i32, height: i32) -> Grid {
        let mut grid = Grid {
            width,
            height,
            cells: Vec::new(),
            backtrace: Vec::new(),
            current: 0,
        };

        for row in 0..height {
            for column in 0..width {
                grid.cells.push(Cell::new(row, column));
            }
        }

        grid
    }

    fn calculate_index(&self, row: i32, column: i32) -> i32 {
        if row < 0 || column < 0 || row > self.height - 1 || column > self.width - 1 {
            -1
        } else {
            column + (row * self.width)
        }
    }

    fn get_available_neighbours(&self) -> Vec<usize> {
        let mut neighbours = Vec::<usize>::new();

        let current_row = self.cells[self.current].row;
        let current_column = self.cells[self.current].column;

        let neighbour_indices: [i32; 4] = [
            self.calculate_index(current_row - 1, current_column),
            self.calculate_index(current_row, current_column + 1),
            self.calculate_index(current_row + 1, current_column),
            self.calculate_index(current_row, current_column - 1),
        ];

        for i in neighbour_indices.iter() {
            if *i != -1 && !self.cells[*i as usize].visited {
                neighbours.push(*i as usize);
            }
        }

        neighbours
    }

    fn find_next_cell(&mut self) -> Option<usize> {
        let neighbours = self.get_available_neighbours();
        if neighbours.is_empty() {
            return None;
        }

        if neighbours.len() == 1 {
            return Some(neighbours[0]);
        }

        let mut rng = RandomGen::default();
        Some(neighbours[rng.range(0, neighbours.len())])
    }

    fn generate_maze(&mut self, data: &mut BuilderMap) {
        let mut i = 0;
        loop {
            self.cells[self.current].visited = true;
            let next = self.find_next_cell();

            match next {
                Some(next) => {
                    self.cells[next].visited = true;
                    self.backtrace.push(self.current);

                    let (lower_part, higher_part) =
                        self.cells.split_at_mut(std::cmp::max(self.current, next));
                    let cell1 = &mut lower_part[std::cmp::min(self.current, next)];
                    let cell2 = &mut higher_part[0];

                    cell1.remove_walls(cell2);
                    self.current = next;
                }
                None => {
                    if !self.backtrace.is_empty() {
                        self.current = self.backtrace[0];
                        self.backtrace.remove(0);
                    } else {
                        break;
                    }
                }
            }
            if i % 50 == 0 {
                self.copy_to_map(&mut data.map);
                data.take_snapshot();
            }
            i += 1;
        }
    }

    fn copy_to_map(&self, map: &mut Map) {
        for i in map.tiles.iter_mut() {
            i.surface = Surface::Wall;
        }
        for cell in self.cells.iter() {
            let p = Point::new(cell.column + 1, cell.row + 1);

            let idx = map.point2d_to_index(p * 2);
            map.tiles[idx].surface = Surface::Floor;
            if !cell.walls[TOP] {
                map.tiles[idx - map.width as usize].surface = Surface::Floor
            }
            if !cell.walls[RIGHT] {
                map.tiles[idx + 1].surface = Surface::Floor
            }
            if !cell.walls[BOTTOM] {
                map.tiles[idx + map.width as usize].surface = Surface::Floor
            }
            if !cell.walls[LEFT] {
                map.tiles[idx - 1].surface = Surface::Floor
            }
        }
    }
}

pub struct MazeBuilder {}

impl MazeBuilder {
    #[allow(dead_code)]
    pub fn new() -> Box<MazeBuilder> {
        Box::new(MazeBuilder {})
    }
}

impl super::InitialMapBuilder for MazeBuilder {
    fn build_map(&mut self, data: &mut super::BuilderMap) {
        let mut maze = Grid::new((data.map.width / 2) - 2, (data.map.height / 2) - 2);
        maze.generate_maze(data);
    }
}
