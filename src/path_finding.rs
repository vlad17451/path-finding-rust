use bevy::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

const ORTOGONAL_COST: f32 = 10.;
const DIAGONAL_COST: f32 = 14.142;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub cost: f32,
    pub goal_distance: f32, // эврестическое приближение
    pub direction: u8,      // 0 = unknown, 1 - 8 = directions
}

impl Cell {
    pub fn get_total_cost(&self) -> f32 {
        self.cost + self.goal_distance
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrderedFloat(f32);

impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Eq for OrderedFloat {}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

#[derive(Resource, Debug, Clone)]
pub struct PathFinding {
    pub height: u32,
    pub width: u32,

    pub start: (u32, u32),
    pub target: (u32, u32),

    pub open_array: BinaryHeap<(OrderedFloat, (u32, u32))>, // (cost, position)
    pub closed_array: HashSet<(u32, u32)>,
    pub cell_map: HashMap<(u32, u32), Cell>,

    pub removed_nodes: HashSet<(u32, u32)>, // TODO
    pub finished: bool,

    pub path: Vec<(u32, u32)>,

    pub walls: Vec<Vec<u32>>,
}

impl PathFinding {
    pub fn new(
        start: (u32, u32),
        target: (u32, u32),
        walls: Vec<Vec<u32>>,
        height: u32,
        width: u32,
    ) -> Self {
        Self {
            path: vec![],
            height,
            width,
            finished: false,
            walls,
            open_array: BinaryHeap::new(),
            closed_array: HashSet::new(),
            removed_nodes: HashSet::new(),
            cell_map: HashMap::new(),
            start,
            target,
        }
    }

    fn is_available(&self, pos: (i32, i32)) -> bool {
        let x = pos.0;
        let y = pos.1;
        let out_of_bounds = x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32;
        if out_of_bounds {
            return false;
        }
        let is_wall = self.walls[y as usize][x as usize] == 1;
        return !is_wall;
    }

    fn push_to_open_set(&mut self, cost: f32, pos: (u32, u32)) {
        // Negate cost to convert max-heap to min-heap behavior
        self.open_array.push((OrderedFloat(-cost), pos));
    }

    pub fn generate(&mut self) {
        if self.start == self.target {
            self.finished = true;
            self.path = vec![self.start];
            return;
        }
        if !self.is_available((self.target.0 as i32, self.target.1 as i32)) {
            return;
        }

        let start_cell = Cell {
            cost: 0.,
            goal_distance: get_goal_distance(&self.start, &self.target),
            direction: 0,
        };
        self.cell_map.insert(self.start, start_cell);
        self.push_to_open_set(start_cell.get_total_cost(), self.start);

        let max_iterations = self.width * self.height;
        let mut iterations = 0;
        while !self.finished && iterations < max_iterations {
            self.scan_neighbours();
            iterations += 1;
        }
    }

    fn get_path(&self, &from: &(u32, u32), current_path: &mut Vec<(u32, u32)>) -> Vec<(u32, u32)> {
        if !self.finished {
            return vec![];
        }
        if from == self.start {
            return current_path.to_vec();
        }

        let from_cell = self.cell_map.get(&from);
        let Some(from_cell) = from_cell else {
            return current_path.to_vec();
        };
        let direction = from_cell.direction;
        let new_pos: (i32, i32) = match direction {
            1 => (-1, 1),
            2 => (0, 1),
            3 => (1, 1),
            4 => (-1, 0),
            5 => (1, 0),
            6 => (-1, -1),
            7 => (0, -1),
            8 => (1, -1),
            _ => (from.0 as i32, from.1 as i32),
        };
        let next_cell = (
            (from.0 as i32 + new_pos.0) as u32,
            (from.1 as i32 + new_pos.1) as u32,
        );

        current_path.push(next_cell);

        return self.get_path(&next_cell, current_path);
    }

    pub fn scan_neighbours(&mut self) {
        if self.finished || self.open_array.is_empty() {
            return;
        }

        let scan_pos = if self.open_array.is_empty() {
            let start_cell = Cell {
                cost: 0.,
                goal_distance: get_goal_distance(&self.start, &self.target),
                direction: 0,
            };
            self.cell_map.insert(self.start, start_cell);
            self.push_to_open_set(start_cell.get_total_cost(), self.start);
            self.start
        } else {
            let Some((_, pos)) = self.open_array.pop() else {
                return;
            };
            pos
        };

        // Get neighbors
        let x = scan_pos.0 as i32;
        let y = scan_pos.1 as i32;
        let neighbours = vec![
            (x - 1, y + 1), // top left
            (x, y + 1),     // top
            (x + 1, y + 1), // top right
            (x - 1, y),     // left
            (x + 1, y),     // right
            (x - 1, y - 1), // bottom left
            (x, y - 1),     // bottom
            (x + 1, y - 1), // bottom right
        ];

        let Some(current_cell) = self.cell_map.get(&scan_pos).cloned() else {
            return;
        };

        for new_pos in neighbours {
            if !self.is_available(new_pos) {
                continue;
            }
            let new_pos = (new_pos.0 as u32, new_pos.1 as u32);

            if self.closed_array.contains(&new_pos) {
                continue;
            }

            let new_direction = get_direction(&new_pos, &scan_pos);
            let new_direction_cost = get_cost_by_direction(new_direction);

            let new_cell = Cell {
                cost: current_cell.cost + new_direction_cost,
                goal_distance: get_goal_distance(&new_pos, &self.target),
                direction: new_direction,
            };

            // Check if target is found
            if new_pos == self.target {
                self.cell_map.insert(new_pos, new_cell);
                self.closed_array.insert(scan_pos);
                self.finished = true;
                break;
            }

            // Update cell if it's better than existing one
            match self.cell_map.get(&new_pos) {
                Some(existing_cell) => {
                    let existing_total_cost = existing_cell.get_total_cost();
                    let new_total_cost = new_cell.get_total_cost();
                    if new_total_cost >= existing_total_cost {
                        continue;
                    }
                    // Remove the old entry from open_array before adding the new one
                    self.open_array.retain(|(_, pos)| pos != &new_pos);
                    self.cell_map.insert(new_pos, new_cell);
                    self.push_to_open_set(new_total_cost, new_pos);
                }
                None => {
                    self.cell_map.insert(new_pos, new_cell);
                    self.push_to_open_set(new_cell.get_total_cost(), new_pos);
                }
            }
        }

        self.closed_array.insert(scan_pos);

        if self.finished {
            self.path = self.get_path(&self.target, &mut vec![self.target]);
        }
    }
}

fn get_goal_distance(from: &(u32, u32), to: &(u32, u32)) -> f32 {
    let dx = (from.0 as i32 - to.0 as i32).abs() as f32;
    let dy = (from.1 as i32 - to.1 as i32).abs() as f32;
    let diagonal = dx.min(dy);
    let straight = (dx - diagonal).abs() + (dy - diagonal).abs();
    diagonal * DIAGONAL_COST + straight * ORTOGONAL_COST
}

fn get_cost_by_direction(direction: u8) -> f32 {
    match direction {
        1 | 3 | 6 | 8 => DIAGONAL_COST,
        2 | 4 | 5 | 7 => ORTOGONAL_COST,
        _ => 0.,
    }
}

fn get_direction(from: &(u32, u32), to: &(u32, u32)) -> u8 {
    // 1 2 3
    // 4 0 5
    // 6 7 8

    if from == to {
        return 0;
    }

    let top = from.1 < to.1;
    let bottom = from.1 > to.1;
    let right = from.0 < to.0;
    let left = from.0 > to.0;

    if top {
        if left {
            return 1;
        }
        if right {
            return 3;
        }
        return 2;
    }
    if bottom {
        if left {
            return 6;
        }
        if right {
            return 8;
        }
        return 7;
    }
    if left {
        return 4;
    }
    if right {
        return 5;
    }
    return 0;
}
