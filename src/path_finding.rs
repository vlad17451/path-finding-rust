use bevy::prelude::*;
use std::collections::HashMap;

const ORTOGONAL_COST: u32 = 10;
const DIAGONAL_COST: u32 = 14;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub cost: u32,
    pub goal_distance: u32, // эврестическое приближение
    pub direction: u8, // 0 = unknown, 1 - 8 = directions
}

impl Cell {
    pub fn get_total_cost(&self) -> u32 {
        self.cost + self.goal_distance
    }
}

// TODO replace Vec2 with (i32, i32)
#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: i32,
    y: i32,
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

    pub fn get_direction(
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

    fn is_wall(&self, pos: (i32, i32)) -> bool {
        let x = pos.0;
        let y = pos.1;
        let out_of_bounds = x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32;
        if out_of_bounds {
            return true;
        }
        let is_wall = self.location[y as usize][x as usize] == 1;
        return is_wall;
    }

    pub fn scan_neighbours(
        &mut self
    ) {

        let scan_pos: (u32, u32);
        if self.open_array.len() == 0 {
            let start_cell = Cell {
                cost: 0,
                goal_distance: get_goal_distance(&self.start, &self.target),
                direction: 0,
            };
            self.open_array.push(self.start);
            self.cell_map.insert(self.start, start_cell);

            scan_pos = self.start;
        } else {
            let best_cell = get_best_cell(&self);
            if let Some(best_cell) = best_cell {
                scan_pos = best_cell;
            } else {
                return;
            }
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
    
        for neighbour_pos in neighbours {
            println!("neighbour_pos: {:?}", neighbour_pos);
    
            if self.is_wall(neighbour_pos) {
                continue;
            }
            let neighbour_pos = (neighbour_pos.0 as u32, neighbour_pos.1 as u32);
    
            // let neighbour_index = vec2_to_index(&neighbour_pos);
            let neighbour_cell = self.cell_map.get(&neighbour_pos);
    
            
            let new_direction = PathFinding::get_direction(&neighbour_pos, &scan_pos);
            // let new_goal_distance = get_goal_distance(&neighbour_pos, &self.target);
            let goal_distance = get_goal_distance(&neighbour_pos, &self.target);
    
            let target_found = goal_distance == 0;
    
            if target_found {
                
                // TODO move current cell to closed array from open array
                self.closed_array.push(scan_pos);
                self.closed_array.push(neighbour_pos);
                self.open_array.retain(|&x| x != scan_pos);
                self.finished = true;
    
                let final_cell = Cell {
                    cost: current_cell.cost + get_cost_by_direction(new_direction),
                    goal_distance,
                    direction: PathFinding::get_direction(&neighbour_pos, &scan_pos),
                };
    
                // println!("Final cell: {:?}", final_cell);
    
                self.cell_map.insert(self.target, final_cell);
    
                // println!("Path found");
                return;
            }
    
    
    
            let new_cost = get_cost_by_direction(new_direction);
            let new_cell = Cell {
                cost: current_cell.cost + new_cost, // cost of scan_pos + direction cost
                goal_distance,
                direction: new_direction,
            };
    
    
            if let Some(neighbour_cell) = neighbour_cell {
                if new_cell.get_total_cost() < neighbour_cell.get_total_cost() {
                    self.cell_map.insert(neighbour_pos, new_cell);
                }
            } else {
                self.cell_map.insert(neighbour_pos, new_cell);
                self.open_array.push(neighbour_pos);
            }
        }
    
        // TODO move current cell to closed array from open array
        self.closed_array.push(scan_pos);
        self.open_array.retain(|&x| x != scan_pos);
    
    
        println!("Direction: {:?} ", self.cell_map.get(&scan_pos));
    }

    // fn get_cell_by_pos(&self, position: Vec2) -> Option<Cell> {
    //     let index = vec2_to_index(&position);
    //     let cell = self.cell_map.get(&index);
    //     if let Some(cell) = cell {
    //         return Some(cell.clone());
    //     }
    //     return None;
    // }
}

fn get_goal_distance(from: &(u32, u32), to: &(u32, u32)) -> u32 {
    let dx = (from.0 as i32 - to.0 as i32).abs();
    let dy = (from.1 as i32 - to.1 as i32).abs();
    (dx + dy) as u32 * 10
}

fn get_cost_by_direction(
    direction: u8
) -> u32 {
    match direction {
        1 | 3 | 5 | 7 => DIAGONAL_COST,
        2 | 4 | 6 | 8 => ORTOGONAL_COST,
        _ => 0,
    }
}

// fn vec2_to_index(vec2: &Vec2) -> (u32, u32) {
//     (vec2.x as u32, vec2.y as u32)
// }

fn get_best_cell(meta: &PathFinding) -> Option<(u32, u32)> {
    if meta.open_array.len() == 0 {
        return None;
    }
    if meta.open_array.len() == 1 {
        return Some(meta.open_array[0]);
    }

    let mut best_pos: (u32, u32) = meta.open_array[0];
    for pos_to_check in &meta.open_array {
        let cell = meta.cell_map.get(&pos_to_check);
        let best_cell = meta.cell_map.get(&best_pos);
        
        if let Some(cell) = cell {
            if let Some(best_cell) = best_cell {
                if cell.get_total_cost() < best_cell.get_total_cost() {
                    best_pos = pos_to_check.clone();
                }
            }
        }
    }
    return Some(best_pos);
}