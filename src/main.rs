use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};

mod path_finding;

use path_finding::*;

const X_SIZE: u32 = 20;
const Y_SIZE: u32 = 20;

const SPEED: f32 = 7.0; // cells per second

const WALLS: [[u32; Y_SIZE as usize]; X_SIZE as usize] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0],
    [0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0],
    [0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0],
    [0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0],
    [0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0],
    [0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
];

const START: Vec2 = Vec2::new(4., 0.7);

const CELL_SIZE: f32 = 30.;
const GRID_SIZE: Vec2 = Vec2::new(X_SIZE as f32, Y_SIZE as f32);

const GRID_HALF_SIZE: Vec2 = Vec2::new(
    X_SIZE as f32 * CELL_SIZE / 2.,
    Y_SIZE as f32 * CELL_SIZE / 2.,
);

#[derive(Resource)]
struct Unit {
    // target: Option<(u32, u32)>,
    path_finding: Option<PathFinding>,
    pos: Vec2,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PanCamPlugin::default()))
        .insert_resource(Unit {
            // target: None,
            path_finding: None,
            pos: START,
        })
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, button_system)
        .add_systems(Update, render_grid)
        .add_systems(Update, render_walls)
        .add_systems(Update, render_arrows)
        .add_systems(Update, render_unit)
        .add_systems(Update, unit_system)
        .run();
}

fn render_unit(mut gizmos: Gizmos, unit: Res<Unit>) {
    gizmos.circle_2d(
        unit.pos * CELL_SIZE - GRID_HALF_SIZE,
        CELL_SIZE / 2. - 7.,
        Color::srgba(0.5, 0.5, 0.9, 1.),
    );
}

fn vec2_to_index(vec2: &Vec2) -> (u32, u32) {
    (vec2.x.round() as u32, vec2.y.round() as u32)
}

fn button_system(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut gizmos: Gizmos,
    mut unit: ResMut<Unit>,
) {
    // if unit.path_finding.is_some() {
    //     return;
    // }

    if buttons.pressed(MouseButton::Right) {
        unit.path_finding = None;
        return;
    }

    if !buttons.pressed(MouseButton::Left) {
        return;
    }

    let (camera, camera_transform) = camera_query.single();
    let Some(cursor_screen_pos) = windows.single().cursor_position() else {
        return;
    };
    let Some(point) = camera
        .viewport_to_world_2d(camera_transform, cursor_screen_pos)
        .ok()
    else {
        return;
    };

    gizmos.circle_2d(point, 10., Color::WHITE);

    let x = ((point.x + GRID_HALF_SIZE.x) / CELL_SIZE).round() as u32;
    let y = ((point.y + GRID_HALF_SIZE.y) / CELL_SIZE).round() as u32;

    let mut path_finding = PathFinding::new(
        vec2_to_index(&unit.pos),
        (x, y),
        get_walls(),
        X_SIZE,
        Y_SIZE,
    );
    path_finding.generate();

    unit.path_finding = Some(path_finding);
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        PanCam {
            grab_buttons: vec![MouseButton::Right, MouseButton::Middle],
            ..default()
        },
    ));
}

fn render_arrows(mut gizmos: Gizmos, unit: Res<Unit>) {
    let Some(path_finding) = &unit.path_finding else {
        return;
    };
    for &(_, pos) in &path_finding.open_array {
        render_rect(
            Vec2::new(pos.0 as f32, pos.1 as f32),
            Vec2::splat(CELL_SIZE - 7.),
            Color::srgba(1.0, 1.0, 1.0, 0.3),
            &mut gizmos,
        );
    }

    let max_goal_distance = path_finding
        .cell_map
        .values()
        .map(|c| c.goal_distance)
        .fold(0.0 / 0.0, f32::max); // Get the maximum goal_distance

    for &pos in &path_finding.closed_array {
        let Some(cell) = path_finding.cell_map.get(&pos) else {
            continue;
        };

        let intensity = 1.0 - (cell.goal_distance / max_goal_distance).clamp(0.0, 1.0);

        render_rect(
            Vec2::new(pos.0 as f32, pos.1 as f32),
            Vec2::splat(CELL_SIZE - 7.),
            Color::srgb(1.0, intensity, 0.0),
            &mut gizmos,
        );
    }

    let all_positions = path_finding.closed_array.iter();
    // let all_positions = path.open_array.iter().chain(path.closed_array.iter());
    for pos in all_positions {
        let cell = path_finding.cell_map.get(&pos);
        if let Some(cell) = cell {
            let start = Vec2::new(pos.0 as f32, pos.1 as f32) * CELL_SIZE - GRID_HALF_SIZE;
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
            gizmos.arrow_2d(start, end, Color::srgb(0.8, 0.4, 0.1));
        }
    }

    if path_finding.finished {
        let path = &path_finding.path;

        for pos in path {
            render_rect(
                Vec2::new(pos.0 as f32, pos.1 as f32),
                Vec2::splat(CELL_SIZE - 15.),
                Color::srgb(0.1, 0.1, 1.0),
                &mut gizmos,
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
                render_rect(
                    Vec2::new(x as f32, y as f32),
                    Vec2::splat(CELL_SIZE - 7.),
                    Color::srgb(0.2, 0.8, 0.5),
                    &mut gizmos,
                );
            }
        }
    }
}

fn render_rect(vector: Vec2, size: Vec2, color: Color, gizmos: &mut Gizmos) {
    gizmos.rect_2d(vector * CELL_SIZE - GRID_HALF_SIZE, size, color);
}

fn render_grid(mut gizmos: Gizmos) {
    for x in 0..GRID_SIZE.x as i32 {
        for y in 0..GRID_SIZE.y as i32 {
            render_rect(
                Vec2::new(x as f32, y as f32),
                Vec2::splat(CELL_SIZE),
                Color::srgb(0.5, 0.5, 0.5),
                &mut gizmos,
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

    let speed = SPEED * time.delta_secs();
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
