
use bevy::prelude::*;


mod path_finding;

use path_finding::*;

const X_SIZE: u32 = 10;
const Y_SIZE: u32 = 10;

const START: Vec2 = Vec2::new(0., 0.);
const TARGET: Vec2 = Vec2::new(9., 9.);


const CELL_SIZE: f32 = 50.;
const GRID_SIZE: Vec2 = Vec2::new(X_SIZE as f32, Y_SIZE as f32);


const GRID_HALF_SIZE: Vec2 = Vec2::new(X_SIZE as f32 * CELL_SIZE / 2., Y_SIZE as f32 * CELL_SIZE / 2.);

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


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(
            PathFinding::new(vec2_to_index(&START), vec2_to_index(&TARGET), get_location(), X_SIZE, Y_SIZE)
        )
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
    // mut query: Query<&mut Text, With<Scoreboard>>,
    // path: Res<PathFinding>
) {
    // let mut text = query.single_mut();
    // text.sections[0].value = format!("Age: {}", path.age);
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


fn vec2_to_index(vec2: &Vec2) -> (u32, u32) {
    (vec2.x as u32, vec2.y as u32)
}

fn button_system(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>),
    >,
    mut path: ResMut<PathFinding>,
    time: Res<Time>,
) {
    // for interaction in &mut interaction_query {
    //     if *interaction == Interaction::Pressed {
            
    //         if path.finished {
    //             println!("Path already found");
    //             return;
    //         }
            // path.scan_neighbours();
    //     }
    // }

    // TODO each 100ms do path.scan_neighbours();
    // if time. % 0.1 < 0.01 {
        println!("Time: {}", time.elapsed_seconds() % 0.2);
    if time.elapsed_seconds() % 0.2 < 0.1 {
        path.scan_neighbours();
    }
    // }

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
    mut gizmos: Gizmos,
    path: Res<PathFinding>
) {
    gizmos.rect_2d(
        // path.start * CELL_SIZE - GRID_HALF_SIZE,
        Vec2::new(path.start.0 as f32, path.start.1 as f32) * CELL_SIZE - GRID_HALF_SIZE,
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
    path: Res<PathFinding>
) {
    for &pos in &path.open_array {
        gizmos.rect_2d(
            // possition * CELL_SIZE - GRID_HALF_SIZE,
            Vec2::new(pos.0 as f32, pos.1 as f32) * CELL_SIZE - GRID_HALF_SIZE,
            0.,
            Vec2::splat(CELL_SIZE - 7.),
            Color::Rgba { red: 0.9, green: 0.9, blue: 0.5, alpha: 1. },
        );
    }
    for &pos in &path.closed_array {
        gizmos.rect_2d(
            Vec2::new(pos.0 as f32, pos.1 as f32) * CELL_SIZE - GRID_HALF_SIZE,
            0.,
            Vec2::splat(CELL_SIZE - 7.),
            Color::Rgba { red: 0.9, green: 0.5, blue: 0.9, alpha: 1. },
        );
    }

    // TODO combine open and closed arrays
    let all_positions = path.closed_array.iter();
    // let all_positions = path.open_array.iter().chain(path.closed_array.iter());
    for pos in all_positions {
        let cell = path.cell_map.get(&pos);
        if let Some(cell) = cell {

            let start = Vec2::new(pos.0 as f32, pos.1 as f32) * CELL_SIZE - GRID_HALF_SIZE;
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

    if path.finished {
        let path = get_path(
            &path.target,
            &mut vec![],
            &path
        );
        
        for pos in path {
            gizmos.rect_2d(
                // cell * CELL_SIZE - GRID_HALF_SIZE,
                Vec2::new(pos.0 as f32, pos.1 as f32) * CELL_SIZE - GRID_HALF_SIZE,
                0.,
                Vec2::splat(CELL_SIZE - 15.),
                Color::Rgba { red: 0.1, green: 0.1, blue: 1.0, alpha: 1. },
            );
        }
        
    }
}

fn get_path(
    &from: &(u32, u32),
    current_path: &mut Vec<(u32, u32)>,
    path: &PathFinding
) -> Vec<(u32, u32)> {
    if !path.finished {
        return vec![];
    }
    if from == path.start {
        return (&current_path).to_vec();
    }

    // TODO get cell depending of direction of FROM cell
    let from_cell = path.cell_map.get(&from);
    let Some(from_cell) = from_cell else {
        return (&current_path).to_vec();
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

    return get_path(
        &next_cell,
        current_path,
        path
    );

}

fn get_location() -> Vec<Vec<u32>> {
    let mut copy: Vec<Vec<u32>> = LOCATION.iter().map(|row| row.to_vec()).collect();
    copy.reverse();
    copy
}

fn render_walls(
    mut gizmos: Gizmos
) {
    
    let location = get_location();

    for y in 0..Y_SIZE {
        for x in 0..X_SIZE {
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
