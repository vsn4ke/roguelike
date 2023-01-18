//A* implementation from bracket-lib, modified to try less steps

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::convert::TryInto;

use bracket_lib::prelude::BaseMap;

const MAX_ASTAR_STEPS: usize = 200;

pub fn a_star_search<T>(start: T, end: T, map: &dyn BaseMap) -> NavigationPath
where
    T: TryInto<usize>,
{
    AStar::new(start.try_into().ok().unwrap(), end.try_into().ok().unwrap()).search(map)
}

#[derive(Clone, Default)]
pub struct NavigationPath {
    pub destination: usize,
    pub success: bool,
    pub steps: Vec<usize>,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
struct Node {
    idx: usize,
    f: f32,
    g: f32,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, b: &Self) -> Ordering {
        b.f.partial_cmp(&self.f).unwrap()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, b: &Self) -> Option<Ordering> {
        b.f.partial_cmp(&self.f)
    }
}

impl NavigationPath {
    pub fn new() -> NavigationPath {
        NavigationPath {
            destination: 0,
            success: false,
            steps: Vec::new(),
        }
    }
}

struct AStar {
    start: usize,
    end: usize,
    open_list: BinaryHeap<Node>,
    closed_list: HashMap<usize, f32>,
    parents: HashMap<usize, (usize, f32)>, // (index, cost)
    step_counter: usize,
}

impl AStar {
    fn new(start: usize, end: usize) -> AStar {
        let mut open_list: BinaryHeap<Node> = BinaryHeap::new();
        open_list.push(Node {
            idx: start,
            f: 0.0,
            g: 0.0,
        });

        AStar {
            start,
            end,
            open_list,
            parents: HashMap::new(),
            closed_list: HashMap::new(),
            step_counter: 0,
        }
    }

    fn distance_to_end(&self, idx: usize, map: &dyn BaseMap) -> f32 {
        map.get_pathing_distance(idx, self.end)
    }

    fn add_successor(&mut self, q: Node, idx: usize, cost: f32, map: &dyn BaseMap) {
        let distance_to_end = self.distance_to_end(idx, map);
        let s = Node {
            idx,
            f: q.g + cost + distance_to_end,
            g: cost,
        };

        let mut should_add = true;
        if let Some(e) = self.parents.get(&idx) {
            if e.1 < s.g {
                should_add = false;
            }
        }
        if should_add && self.closed_list.contains_key(&idx) {
            should_add = false;
        }

        if should_add {
            self.open_list.push(s);
            self.parents.insert(idx, (q.idx, s.g));
        }
    }

    fn found_it(&self) -> NavigationPath {
        let mut result = NavigationPath::new();
        result.success = true;
        result.destination = self.end;

        result.steps.push(self.end);
        let mut current = self.end;
        while current != self.start {
            let parent = self.parents[&current];
            result.steps.insert(0, parent.0);
            current = parent.0;
        }

        result
    }

    fn search(&mut self, map: &dyn BaseMap) -> NavigationPath {
        let result = NavigationPath::new();
        while !self.open_list.is_empty() && self.step_counter < MAX_ASTAR_STEPS {
            self.step_counter += 1;

            let q = self.open_list.pop().unwrap();
            if q.idx == self.end {
                let success = self.found_it();
                return success;
            }

            map.get_available_exits(q.idx)
                .iter()
                .for_each(|s| self.add_successor(q, s.0, s.1, map));

            if self.closed_list.contains_key(&q.idx) {
                self.closed_list.remove(&q.idx);
            }
            self.closed_list.insert(q.idx, q.f);
        }
        result
    }
}
