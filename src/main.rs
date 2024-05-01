use std::collections::HashMap;

use bevy::prelude::*;

// const WALL_CELLS: [(i32, i32); 6] = [
//     (1, 4),
//     (2, 4),
//     (3, 4),
//     (4, 4),
//     (5, 6),
//     (5, 3),
// ];

const X_SIZE: u32 = 10;
const Y_SIZE: u32 = 10;

    // TODO create 2d array of walls, with 0 and 1 values
const LOCATION: [[u32; Y_SIZE as usize]; X_SIZE as usize] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 1, 1, 1, 1, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 1, 1, 0, 1, 0],
    [0, 0, 0, 0, 0, 1, 0, 0, 1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 1, 1, 1, 1, 0, 0, 1, 0],
    [0, 0, 0, 0, 0, 1, 1, 0, 1, 0],
    [0, 0, 0, 0, 0, 0, 1, 0, 1, 0],
    [0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 1, 0, 0],
];

const AGENT_START: Vec2 = Vec2::new(0., 0.);
const TARGET: Vec2 = Vec2::new(9., 9.);


const CELL_SIZE: f32 = 50.;
const GRID_SIZE: Vec2 = Vec2::new(X_SIZE as f32, Y_SIZE as f32);


const GRID_HALF_SIZE: Vec2 = Vec2::new(X_SIZE as f32 * CELL_SIZE / 2., Y_SIZE as f32 * CELL_SIZE / 2.);

const ORTOGONAL_COST: u32 = 10;
const DIAGONAL_COST: u32 = 14;


#[derive(Debug, Clone, Copy)]
struct Cell {
    cost: u32,
    goal_distance: u32, // эврестическое приближение
    direction: u8, // 0 = unknown, 1 - 8 = directions
    // position: Vec2,
}

impl Cell {
    fn get_total_cost(&self) -> u32 {
        self.cost + self.goal_distance
    }
}


#[derive(Resource, Debug, Clone)]
struct Meta {
    age: u32,

    start: Vec2,
    target: Vec2,

    open_array: Vec<Vec2>,
    closed_array: Vec<Vec2>,
    cell_map: HashMap<(i32,i32), Cell>,

    finished: bool,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            finished: false,
            age: 0,
            open_array: vec![],
            closed_array: vec![],
            cell_map: HashMap::new(),
            start: AGENT_START,
            target: TARGET,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Meta>()
        .add_systems(Startup, setup)

        .add_systems(Startup, setup_buttons)
        .add_systems(Update, button_style)
        .add_systems(FixedUpdate, button_system)

        .add_systems(Startup, setup_scoreboard)
        .add_systems(Update, scoreboard_system)

        .add_systems(Update, render_grid)
        .add_systems(Update, render_walls)
        .add_systems(Update, render_agent)
        .add_systems(Update, render_arrays)
        .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);


#[derive(Component, Debug)]
struct Scoreboard;

fn setup_scoreboard(mut commands: Commands) {
    commands.spawn((
       TextBundle::from_section(
           "",
           TextStyle {
               font_size: 40.0,
               color: Color::WHITE,
               ..default()
           },
       ),
       Scoreboard
   ));
}

fn scoreboard_system(
    mut query: Query<&mut Text, With<Scoreboard>>,
    meta: Res<Meta>
) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("Age: {}", meta.age);
}

fn setup_buttons(
    mut commands: Commands
) {
    commands.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::End,
                justify_content: JustifyContent::End,
                padding: UiRect::all(Val::Px(40.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "+",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}

fn get_goal_distance(from: &Vec2, to: &Vec2) -> u32 {
    let dx = (from.x - to.x).abs();
    let dy = (from.y - to.y).abs();
    (dx + dy) as u32 * 10
}

fn get_best_cell(meta: &Meta) -> Option<Vec2> {
    if meta.open_array.len() == 0 {
        return None;
    }
    if meta.open_array.len() == 1 {
        return Some(meta.open_array[0]);
    }

    let mut best_possition: Vec2 = meta.open_array[0];
    for possition in &meta.open_array {
        let cell = meta.cell_map.get(&vec2_to_index(possition));
        let best_cell = meta.cell_map.get(&vec2_to_index(&best_possition));
        
        if let Some(cell) = cell {
            if cell.get_total_cost() < best_cell.unwrap().get_total_cost() {
                best_possition = possition.clone();
            }
        }
    }
    return Some(best_possition);
}

fn vec2_to_index(vec2: &Vec2) -> (i32, i32) {
    (vec2.x as i32, vec2.y as i32)
}

fn get_direction(
    from: &Vec2,
    to: &Vec2
) -> u8 {

    // 1 2 3
    // 4 0 5
    // 6 7 8
    
    if from == to {
        return 0;
    }

    let top = from.y < to.y;
    let bottom = from.y > to.y;
    let right = from.x < to.x;
    let left = from.x > to.x;

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

fn get_cost_by_direction(
    direction: u8
) -> u32 {
    match direction {
        1 | 3 | 5 | 7 => DIAGONAL_COST,
        2 | 4 | 6 | 8 => ORTOGONAL_COST,
        _ => 0,
    }
}

fn scan_neighbours(
    possition_to_scan: Vec2,
    meta: &mut Meta
) {
    // TODO create 8 neighbours
    let neighbours_possitions = vec![
        Vec2::new(possition_to_scan.x - 1., possition_to_scan.y + 1.), // top left
        Vec2::new(possition_to_scan.x, possition_to_scan.y + 1.),      // top
        Vec2::new(possition_to_scan.x + 1., possition_to_scan.y + 1.), // top right

        Vec2::new(possition_to_scan.x - 1., possition_to_scan.y),      // left
        Vec2::new(possition_to_scan.x + 1., possition_to_scan.y),      // right

        Vec2::new(possition_to_scan.x - 1., possition_to_scan.y - 1.), // bottom left
        Vec2::new(possition_to_scan.x, possition_to_scan.y - 1.),      // bottom
        Vec2::new(possition_to_scan.x + 1., possition_to_scan.y - 1.), // bottom right
        
    ];
    
    let current_cell = meta.cell_map.get(&vec2_to_index(&possition_to_scan));    
    let Some(current_cell) = current_cell else {
        panic!("Current cell is not found");

        // return;
    };
    let current_cell = current_cell.clone();

    // for each neighbour create cell
    // if cell already exists update it if cost is less
    // if cell is new - add it to open array
    for neighbour in neighbours_possitions {
        // if WALL_CELLS.contains(&(neighbour.x as i32, neighbour.y as i32)) {
        //     continue;
        // }
        println!("neighbour: {:?}", neighbour);

        if neighbour.x < 0. || neighbour.x >= X_SIZE as f32 || neighbour.y < 0. || neighbour.y >= Y_SIZE as f32 {
            continue;
        }

        let location = get_location();
        if location[neighbour.y as usize][neighbour.x as usize] == 1 {
            continue;
        }

        let neighbour_index = vec2_to_index(&neighbour);
        let neighbour_cell = meta.cell_map.get(&neighbour_index);

        
        let new_direction = get_direction(&neighbour, &possition_to_scan);
        // let new_goal_distance = get_goal_distance(&neighbour, &meta.target);
        let goal_distance = get_goal_distance(&neighbour, &meta.target);

        if goal_distance == 0 {
            
            // TODO move current cell to closed array from open array
            meta.closed_array.push(possition_to_scan);
            meta.closed_array.push(TARGET);
            meta.open_array.retain(|&x| x != possition_to_scan);
            meta.finished = true;

            let final_cell = Cell {
                cost: current_cell.cost + get_cost_by_direction(new_direction),
                goal_distance,
                direction: get_direction(&neighbour, &possition_to_scan),
            };

            // println!("neighbour: {:?} possition_to_scan: {:?}", neighbour, possition_to_scan);
            println!("Final cell: {:?}", final_cell);

            meta.cell_map.insert(vec2_to_index(&TARGET), final_cell);

            println!("Path found");
            return;
        }



        let new_cost = get_cost_by_direction(new_direction);
        let new_cell = Cell {
            cost: current_cell.cost + new_cost, // cost of possition_to_scan + direction cost
            goal_distance,
            direction: new_direction,
        };


        if let Some(neighbour_cell) = neighbour_cell {
            if new_cell.get_total_cost() < neighbour_cell.get_total_cost() {
                meta.cell_map.insert(neighbour_index, new_cell);
            }
        } else {
            meta.cell_map.insert(neighbour_index, new_cell);
            meta.open_array.push(neighbour);
        }
    }

    // TODO move current cell to closed array from open array
    meta.closed_array.push(possition_to_scan);
    meta.open_array.retain(|&x| x != possition_to_scan);

    // let meta = meta.clone();

    println!("Direction: {:?} ", meta.cell_map.get(&vec2_to_index(&possition_to_scan)));
}

fn button_system(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>),
    >,
    mut meta: ResMut<Meta>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            
            if meta.finished {
                println!("Path already found");
                return;
            }

            let current_cell: Vec2;
            if meta.open_array.len() == 0 {
                let start_cell = Cell {
                    cost: 0,
                    goal_distance: get_goal_distance(&AGENT_START, &TARGET),
                    direction: 0,
                };
                meta.open_array.push(AGENT_START);
                meta.cell_map.insert(vec2_to_index(&AGENT_START), start_cell);

                current_cell = AGENT_START;
            } else {
                let best_cell = get_best_cell(&meta);
                if let Some(best_cell) = best_cell {
                    current_cell = best_cell;
                } else {
                    return;
                }
            }

            scan_neighbours(
                current_cell,
                &mut meta
            );

            meta.age += 1;
        }
    }
}

fn button_style(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>),
    >
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}


fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());

    // TODO add button
}

fn render_agent(
    mut gizmos: Gizmos
) {
    gizmos.rect_2d(
        AGENT_START * CELL_SIZE - GRID_HALF_SIZE,
        0.,
        Vec2::splat(CELL_SIZE - 7.),
        Color::Rgba { red: 0.5, green: 0.5, blue: 0.9, alpha: 1. },
    );
    gizmos.rect_2d(
        TARGET * CELL_SIZE - GRID_HALF_SIZE,
        0.,
        Vec2::splat(CELL_SIZE - 7.),
        Color::Rgba { red: 0.9, green: 0.4, blue: 0.5, alpha: 1. },
    );
}

fn render_arrays(
    mut gizmos: Gizmos,
    meta: Res<Meta>
) {
    for &possition in &meta.open_array {
        gizmos.rect_2d(
            possition * CELL_SIZE - GRID_HALF_SIZE,
            0.,
            Vec2::splat(CELL_SIZE - 7.),
            Color::Rgba { red: 0.9, green: 0.9, blue: 0.5, alpha: 1. },
        );
    }
    for &possition in &meta.closed_array {
        gizmos.rect_2d(
            possition * CELL_SIZE - GRID_HALF_SIZE,
            0.,
            Vec2::splat(CELL_SIZE - 7.),
            Color::Rgba { red: 0.9, green: 0.5, blue: 0.9, alpha: 1. },
        );
    }

    // TODO combine open and closed arrays
    let all_positions = meta.closed_array.iter();
    // let all_positions = meta.open_array.iter().chain(meta.closed_array.iter());
    for possition in all_positions {
        let cell = meta.cell_map.get(&vec2_to_index(&possition));
        if let Some(cell) = cell {
            // TODO depend of direction - render arrow in direction
            let start = *possition * CELL_SIZE - GRID_HALF_SIZE;
            // let end = start + Vec2::splat(CELL_SIZE / 2.);
            let end = match cell.direction {
                // top left
                1 => start + Vec2::new(-CELL_SIZE / 2., CELL_SIZE / 2.),
                // top
                2 => start + Vec2::new(0., CELL_SIZE / 2.),
                // top right
                3 => start + Vec2::new(CELL_SIZE / 2., CELL_SIZE / 2.),
                // left
                4 => start + Vec2::new(-CELL_SIZE / 2., 0.),
                // right
                5 => start + Vec2::new(CELL_SIZE / 2., 0.),
                // bottom left
                6 => start + Vec2::new(-CELL_SIZE / 2., -CELL_SIZE / 2.),
                // bottom
                7 => start + Vec2::new(0., -CELL_SIZE / 2.),
                // bottom right
                8 => start + Vec2::new(CELL_SIZE / 2., -CELL_SIZE / 2.),
                _ => start,
            };
            gizmos.arrow_2d(
                start,
                end,
                Color::YELLOW,
            );
        }
    };

    if meta.finished {
        let path = get_path(
            &TARGET,
            &mut vec![],
            &meta
        );
        
        for cell in path {
            gizmos.rect_2d(
                cell * CELL_SIZE - GRID_HALF_SIZE,
                0.,
                Vec2::splat(CELL_SIZE - 15.),
                Color::Rgba { red: 0.1, green: 0.1, blue: 1.0, alpha: 1. },
            );
        }
        
    }
}

fn get_path(
    &from: &Vec2,
    current_path: &mut Vec<Vec2>,
    meta: &Meta
) -> Vec<Vec2> {
    if !meta.finished {
        return vec![];
    }
    if from == AGENT_START {
        return (&current_path).to_vec();
    }

    // TODO get cell depending of direction of FROM cell
    let from_cell = meta.cell_map.get(&vec2_to_index(&from));
    let Some(from_cell) = from_cell else {
        return (&current_path).to_vec();
    };
    let direction = from_cell.direction;
    let new_position = match direction {
        1 => Vec2::new(-1., 1.),
        2 => Vec2::new(0., 1.),
        3 => Vec2::new(1., 1.),
        4 => Vec2::new(-1., 0.),
        5 => Vec2::new(1., 0.),
        6 => Vec2::new(-1., -1.),
        7 => Vec2::new(0., -1.),
        8 => Vec2::new(1., -1.),
        _ => from,
    };
    let next_cell = from + new_position;

    current_path.push(next_cell);

    return get_path(
        &next_cell,
        current_path,
        meta
    );

}

fn get_location() -> [[u32; 10]; 10] {
    let mut copy = LOCATION.clone();
    copy.reverse();
    copy
}

fn render_walls(
    mut gizmos: Gizmos
) {
    // let wall_cells = WALL_CELLS.iter().map(|&(x, y)| Vec2::new(x as f32, y as f32)).collect::<Vec<_>>();
    // for cell in wall_cells {
    //     gizmos.rect_2d(
    //         cell * CELL_SIZE - GRID_HALF_SIZE,
    //         0.,
    //         Vec2::splat(CELL_SIZE - 7.),
    //         Color::Rgba { red: 0.2, green: 0.8, blue: 0.5, alpha: 1. },
    //     );
    // }
    
    let location = get_location();

    // TODO check walls array
    for y in 0..Y_SIZE {
        for x in 0..X_SIZE {
            // TODO reverse first layer of array (LOCATION[y as usize].reverse())
            
            if location[y as usize][x as usize] == 1 {
                gizmos.rect_2d(
                    Vec2::new(x as f32, y as f32) * CELL_SIZE - GRID_HALF_SIZE,
                    0.,
                    Vec2::splat(CELL_SIZE - 7.),
                    Color::Rgba { red: 0.2, green: 0.8, blue: 0.5, alpha: 1. },
                );
            }
        }
    }
}

fn render_grid(
    mut gizmos: Gizmos
) {

    for x in 0..GRID_SIZE.x as i32 {
        for y in 0..GRID_SIZE.y as i32 {
            gizmos.rect_2d(
                Vec2::new(x as f32 * CELL_SIZE, y as f32 * CELL_SIZE) - GRID_HALF_SIZE,
                0.,
                Vec2::splat(CELL_SIZE),
                Color::Rgba { red: 0.5, green: 0.5, blue: 0.5, alpha: 0.1 },
            );
        }
    }
}

// TODO a* pathfinding
