use bevy::prelude::*;

mod path_finding;

use path_finding::*;

// const X_SIZE: u32 = 20; // Doubled from 10
// const Y_SIZE: u32 = 20; // Doubled from 10

// const WALLS: [[u32; Y_SIZE as usize]; X_SIZE as usize] = [
//     [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
//     [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
//     [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
//     [0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
//     [0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0],
//     [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0],
//     [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0],
//     [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
//     [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
//     [0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
//     [0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
//     [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
//     [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
//     [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
//     [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
//     [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
//     [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
//     [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
//     [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
//     [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// ];

const X_SIZE: u32 = 10;
const Y_SIZE: u32 = 10;

const WALLS: [[u32; Y_SIZE as usize]; X_SIZE as usize] = [
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

// const WALLS: [[u32; Y_SIZE as usize]; X_SIZE as usize] = [
//     [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
//     [0, 1, 1, 1, 1, 1, 1, 1, 1, 0],
//     [0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
//     [0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
//     [0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
//     [0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
//     [0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
//     [0, 0, 0, 1, 1, 1, 1, 1, 1, 0],
//     [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
//     [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
// ];

const START: Vec2 = Vec2::new(4., 0.7);
const TARGET: Vec2 = Vec2::new(9., 9.);

const CELL_SIZE: f32 = 50.;
const GRID_SIZE: Vec2 = Vec2::new(X_SIZE as f32, Y_SIZE as f32);

const GRID_HALF_SIZE: Vec2 = Vec2::new(
    X_SIZE as f32 * CELL_SIZE / 2.,
    Y_SIZE as f32 * CELL_SIZE / 2.,
);

#[derive(Resource)]
struct Unit {
    target: Option<(u32, u32)>,
    // path: Vec<(u32, u32)>,
    path_finding: Option<PathFinding>,
    pos: Vec2,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(PathFinding::new(
            vec2_to_index(&START),
            vec2_to_index(&TARGET),
            get_walls(),
            X_SIZE,
            Y_SIZE,
        ))
        .insert_resource(Unit {
            target: None,
            path_finding: None,
            pos: START,
        })
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
        .add_systems(Update, render_unit)
        .add_systems(Update, unit_system)
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
        Scoreboard,
    ));
}

fn render_unit(mut gizmos: Gizmos, unit: Res<Unit>) {
    gizmos.circle_2d(
        unit.pos * CELL_SIZE - GRID_HALF_SIZE,
        CELL_SIZE / 2. - 7.,
        // Vec2::splat(CELL_SIZE - 7.),
        Color::Rgba {
            red: 0.5,
            green: 0.5,
            blue: 0.9,
            alpha: 1.,
        },
    );
}

fn scoreboard_system(mut query: Query<&mut Text, With<Scoreboard>>, path: Res<PathFinding>) {
    // let mut text = query.single_mut();
    // text.sections[0].value = format!("Age: {}", path.age);
}

fn setup_buttons(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
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
    (vec2.x.round() as u32, vec2.y.round() as u32)
}

fn button_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut gizmos: Gizmos,
    // mut cell_map: ResMut<CellMap>,
    mut path: ResMut<PathFinding>,
    // time: Res<Time>,
    mut unit: ResMut<Unit>,
) {
    // if unit.path_finding.is_some() {
    //     return;
    // }

    if !buttons.pressed(MouseButton::Left) {
        return;
    }

    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    gizmos.circle_2d(point, 10., Color::WHITE);

    let x = ((point.x + GRID_HALF_SIZE.x) / CELL_SIZE).round() as u32;
    let y = ((point.y + GRID_HALF_SIZE.y) / CELL_SIZE).round() as u32;
    // println!("Point: {:?} {:?}", x, y);

    let mut path_finding = PathFinding::new(
        vec2_to_index(&unit.pos),
        (x, y),
        get_walls(),
        X_SIZE,
        Y_SIZE,
    );
    path_finding.generate();

    unit.path_finding = Some(path_finding);

    // unit.target = Some(Vec2::new(x as f32, y as f32));
    // path = new_path;

    // for interaction in &interaction_query {
    //     if *interaction == Interaction::Pressed {

    //         if path.finished {
    //             println!("Path already found");
    //             return;
    //         }
    //         path.scan_neighbours();
    //     }
    // }

    // TODO each 100ms do path.scan_neighbours();
    // if time. % 0.1 < 0.01 {
    // println!("Time: {}", time.elapsed_seconds() % 0.2);
    // if time.elapsed_seconds() % 0.05 < 0.01 {
    // path.scan_neighbours();
    // }
    // }
    // path.generate();
}

fn button_style(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
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

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // TODO add button
}

fn render_agent(mut gizmos: Gizmos, path: Res<PathFinding>) {
    gizmos.rect_2d(
        Vec2::new(path.start.0 as f32, path.start.1 as f32) * CELL_SIZE - GRID_HALF_SIZE,
        0.,
        Vec2::splat(CELL_SIZE - 7.),
        Color::Rgba {
            red: 0.5,
            green: 0.5,
            blue: 0.9,
            alpha: 1.,
        },
    );
    gizmos.rect_2d(
        TARGET * CELL_SIZE - GRID_HALF_SIZE,
        0.,
        Vec2::splat(CELL_SIZE - 7.),
        Color::Rgba {
            red: 0.9,
            green: 0.4,
            blue: 0.5,
            alpha: 1.,
        },
    );
}

fn render_arrays(
    mut gizmos: Gizmos,
    // path: Res<PathFinding>
    unit: Res<Unit>,
) {
    let Some(path_finding) = &unit.path_finding else {
        return;
    };
    for &(_, pos) in &path_finding.open_array {
        gizmos.rect_2d(
            Vec2::new(pos.0 as f32, pos.1 as f32) * CELL_SIZE - GRID_HALF_SIZE,
            0.,
            Vec2::splat(CELL_SIZE - 7.),
            Color::Rgba {
                red: 0.9,
                green: 0.9,
                blue: 0.5,
                alpha: 1.,
            },
        );
    }
    for &pos in &path_finding.closed_array {
        gizmos.rect_2d(
            Vec2::new(pos.0 as f32, pos.1 as f32) * CELL_SIZE - GRID_HALF_SIZE,
            0.,
            Vec2::splat(CELL_SIZE - 7.),
            Color::Rgba {
                red: 0.9,
                green: 0.5,
                blue: 0.9,
                alpha: 1.,
            },
        );
    }

    let all_positions = path_finding.closed_array.iter();
    // let all_positions = path.open_array.iter().chain(path.closed_array.iter());
    for pos in all_positions {
        let cell = path_finding.cell_map.get(&pos);
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
            gizmos.arrow_2d(start, end, Color::YELLOW);
        }
    }

    if path_finding.finished {
        let path = &path_finding.path;

        for pos in path {
            gizmos.rect_2d(
                Vec2::new(pos.0 as f32, pos.1 as f32) * CELL_SIZE - GRID_HALF_SIZE,
                0.,
                Vec2::splat(CELL_SIZE - 15.),
                Color::Rgba {
                    red: 0.1,
                    green: 0.1,
                    blue: 1.0,
                    alpha: 1.,
                },
            );
        }
    }
}

fn get_walls() -> Vec<Vec<u32>> {
    let mut copy: Vec<Vec<u32>> = WALLS.iter().map(|row| row.to_vec()).collect();
    copy.reverse();
    copy
}

fn render_walls(mut gizmos: Gizmos) {
    let walls = get_walls();

    for y in 0..Y_SIZE {
        for x in 0..X_SIZE {
            if walls[y as usize][x as usize] == 1 {
                gizmos.rect_2d(
                    Vec2::new(x as f32, y as f32) * CELL_SIZE - GRID_HALF_SIZE,
                    0.,
                    Vec2::splat(CELL_SIZE - 7.),
                    Color::Rgba {
                        red: 0.2,
                        green: 0.8,
                        blue: 0.5,
                        alpha: 1.,
                    },
                );
            }
        }
    }
}

fn render_grid(mut gizmos: Gizmos) {
    for x in 0..GRID_SIZE.x as i32 {
        for y in 0..GRID_SIZE.y as i32 {
            gizmos.rect_2d(
                Vec2::new(x as f32 * CELL_SIZE, y as f32 * CELL_SIZE) - GRID_HALF_SIZE,
                0.,
                Vec2::splat(CELL_SIZE),
                Color::Rgba {
                    red: 0.5,
                    green: 0.5,
                    blue: 0.5,
                    alpha: 0.1,
                },
            );
        }
    }
}

fn unit_system(mut unit: ResMut<Unit>, time: Res<Time>) {
    let (next_step, target) = {
        let Some(path_finding) = &unit.path_finding else {
            return;
        };
        if !path_finding.finished || path_finding.path.is_empty() {
            return;
        }

        (
            path_finding.path[path_finding.path.len() - 1],
            path_finding.target,
        )
    };

    let speed = 3.5 * time.delta_seconds();
    let x_diff = next_step.0 as f32 - unit.pos.x;
    let y_diff = next_step.1 as f32 - unit.pos.y;
    let distance = (x_diff.powi(2) + y_diff.powi(2)).sqrt();

    if distance < speed {
        unit.pos = Vec2::new(next_step.0 as f32, next_step.1 as f32);
        if let Some(path_finding) = &mut unit.path_finding {
            path_finding.path.pop();
        }
    } else {
        unit.pos += Vec2::new(x_diff, y_diff).normalize() * speed;
    }

    if let Some(path_finding) = &unit.path_finding {
        if path_finding.path.is_empty()
            && (unit.pos.x - target.0 as f32).abs() < 0.01
            && (unit.pos.y - target.1 as f32).abs() < 0.01
        {
            unit.path_finding = None;
        }
    }
}
