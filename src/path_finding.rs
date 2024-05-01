use bevy::prelude::*;
use std::collections::HashMap;

const ORTOGONAL_COST: f32 = 10.;
const DIAGONAL_COST: f32 = 14.142;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub cost: f32,
    pub goal_distance: f32, // эврестическое приближение
    pub direction: u8, // 0 = unknown, 1 - 8 = directions
}

impl Cell {
    pub fn get_total_cost(&self) -> f32 {
        self.cost + self.goal_distance
    }
}

#[derive(Resource, Debug, Clone)]
pub struct PathFinding {

    // aria: 

    pub height: u32,
    pub width: u32,

    pub start: (u32, u32),
    pub target: (u32, u32),

    pub open_array: Vec<(u32, u32)>,
    pub closed_array: Vec<(u32, u32)>,
    pub cell_map: HashMap<(u32,u32), Cell>,

    pub finished: bool,

    pub location: Vec<Vec<u32>>,
}

impl PathFinding {
    pub fn new(
        start: (u32, u32),
        target: (u32, u32),
        location: Vec<Vec<u32>>,
        height: u32,
        width: u32,
    ) -> Self {
        Self {
            height,
            width,
            finished: false,
            location,
            open_array: vec![],
            closed_array: vec![],
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
            return true;
        }
        let is_wall = self.location[y as usize][x as usize] == 1;
        return is_wall;
    }

    fn get_best_cell(&self) -> Option<(u32, u32)> {
        match self.open_array.len() {
            0 => None,
            1 => Some(self.open_array[0]),
            _ => {
                let mut best_pos: (u32, u32) = self.open_array[0];
                for pos_to_check in &self.open_array {
                    let cell_to_check = self.cell_map.get(&pos_to_check);
                    let best_cell = self.cell_map.get(&best_pos);

                    let Some(cell_to_check) = cell_to_check else { continue; };
                    let Some(best_cell) = best_cell else { continue; };

                    if cell_to_check.get_total_cost() < best_cell.get_total_cost() {
                        best_pos = pos_to_check.clone();
                    }
                }
                return Some(best_pos);
            }
        }
    }

    pub fn scan_neighbours(
        &mut self
    ) {
        if self.finished {
            return;
        }

        let scan_pos: (u32, u32);
        if self.open_array.len() == 0 {
            let start_cell = Cell {
                cost: 0.,
                goal_distance: get_goal_distance(&self.start, &self.target),
                direction: 0,
            };
            self.open_array.push(self.start);
            self.cell_map.insert(self.start, start_cell);

            scan_pos = self.start;
        } else {
            let best_cell = self.get_best_cell();
            let Some(best_cell) = best_cell else {
                return;
            };
            scan_pos = best_cell;
        }

        // TODO create 8 neighbours
        let x = scan_pos.0 as i32;
        let y = scan_pos.1 as i32;
        let neighbours = vec![
            (x - 1, y + 1), // top left
            (x, y + 1),      // top
            (x + 1, y + 1), // top right
    
            (x - 1, y),      // left
            (x + 1, y),      // right
    
            (x - 1, y - 1), // bottom left
            (x, y - 1),      // bottom
            (x + 1, y - 1), // bottom right
            
        ];
        
        let current_cell = self.cell_map.get(&scan_pos);    
        let Some(current_cell) = current_cell else {
            panic!("Current cell is not found");
            // return;
        };
        let current_cell = current_cell.clone();
    
        for new_pos in neighbours {

            if self.is_available(new_pos) {
                continue;
            }
            let new_pos = (new_pos.0 as u32, new_pos.1 as u32);
    
            let new_direction = get_direction(&new_pos, &scan_pos);
            let new_direction_cost = get_cost_by_direction(new_direction);
 
            let new_cell = Cell {
                cost: current_cell.cost + new_direction_cost,
                goal_distance: get_goal_distance(&new_pos, &self.target),
                direction: new_direction,
            };

            let target_found = new_cell.goal_distance == 0.;
            if target_found {
                self.closed_array.push(new_pos);
                self.finished = true;
            }
    
            let neighbour_cell = self.cell_map.get(&new_pos);


            let Some(neighbour_cell) = neighbour_cell else {
                self.cell_map.insert(new_pos, new_cell);
                self.open_array.push(new_pos);
                continue;
            };

            if new_cell.get_total_cost() < neighbour_cell.get_total_cost() {
                self.cell_map.insert(new_pos, new_cell);
            }
        }
    
        self.closed_array.push(scan_pos);
        self.open_array.retain(|&x| x != scan_pos);
    }
}

fn get_goal_distance(from: &(u32, u32), to: &(u32, u32)) -> f32 {
    let dx = (from.0 as i32 - to.0 as i32).abs();
    let dy = (from.1 as i32 - to.1 as i32).abs();
    (dx + dy) as f32 * ORTOGONAL_COST
}

fn get_cost_by_direction(
    direction: u8
) -> f32 {
    match direction {
        1 | 3 | 6 | 8 => DIAGONAL_COST,
        2 | 4 | 5 | 7 => ORTOGONAL_COST,
        _ => 0.,
    }
}

fn get_direction(
    from: &(u32, u32),
    to: &(u32, u32)
) -> u8 {

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