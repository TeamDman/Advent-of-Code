use common::prelude::*;

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_billboard::prelude::*;
use bevy_xpbd_2d::{math::*, prelude::*};

// 344838 too low

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Advent of Code - Day 03".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(BillboardPlugin)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)),
        )
        .insert_resource(Gravity(Vector::ZERO))
        // .insert_resource(Gravity(Vector::NEG_Y * 100.0))
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.1)))
        .add_event::<MovementAction>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                tag_numbers_touching_symbols,
                keyboard_input,
                movement.run_if(has_movement),
                apply_movement_damping,
            ),
        )
        .run();
}

#[derive(Component, Reflect)]
pub struct Number {
    pub value: usize,
}
#[derive(Component, Reflect)]
pub struct Symbol;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let input = include_str!("input.txt");
    let padding = 1.0;
    let cell_size = 10.0;

    input.lines().enumerate().for_each(|(i, line)| {
        // iterate the line mainCamera by mainCamera
        // we want to create entities for each number and symbol in the line
        // example: ......*...189..591.*.....
        // should create 4 entities:
        // - a symbol entity for the first asterisk
        // - a number entity for the 189
        // - a number entity for the 591
        // - a symbol entity for the second asterisk

        // if the mainCamera is a period, skip it
        // if the mainCamera is numeric, take all the following numerics and spawn a single bundle for the whole number
        // if the mainCamera is not numeric (a "symbol"), spawn a single bundle for that symbol with the Symbol component

        for (j, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }
            let x = j as f32 * (cell_size + padding);
            let y = i as f32 * (cell_size + padding);

            if char.is_numeric() {
                let mut number = char.to_digit(10).unwrap();
                let mut number_string = char.to_string();
                for char in line.chars().skip(j + 1) {
                    if char.is_numeric() {
                        number *= 10;
                        number += char.to_digit(10).unwrap();
                        number_string.push(char);
                    } else {
                        break;
                    }
                }
                commands.spawn((
                    // BillboardTextBundle {
                    //     transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                    //     text: Text::from_sections([TextSection {
                    //         value: number_string,
                    //         style: TextStyle {
                    //             font_size: 10.0,
                    //             font: font.clone(),
                    //             color: Color::GREEN,
                    //         },
                    //     }])
                    //     .with_alignment(TextAlignment::Center),
                    //     ..default()
                    // },
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(0.0, 0.4, 0.7),
                            custom_size: Some(Vec2::new(10.0, 10.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(x, y, 0.0),
                        ..default()
                    },
                    RigidBody::Dynamic,
                    Number {
                        value: number as usize,
                    },
                    Collider::cuboid(10.0, 10.0),
                ));
            } else {
                commands.spawn((
                    // BillboardTextBundle {
                    //     transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                    //     text: Text::from_sections([TextSection {
                    //         value: char.to_string(),
                    //         style: TextStyle {
                    //             font_size: 20.0,
                    //             font: font.clone(),
                    //             color: Color::RED,
                    //         },
                    //     }])
                    //     .with_alignment(TextAlignment::Center),
                    //     ..default()
                    // },
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(0.7, 0.4, 0.0),
                            custom_size: Some(Vec2::new(20.0, 20.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(x, y, 0.0),
                        ..default()
                    },
                    RigidBody::Dynamic,
                    Symbol,
                    Sensor,
                    Collider::cuboid(20.0, 20.0),
                ));
            }
        }
    });

    // Camera2d
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(
                input.lines().nth(0).iter().count() as f32 / 2.0 * (cell_size + padding),
                0.0,
                0.0,
            ),
            ..default()
        },
        MassPropertiesBundle {
            mass: Mass(25.0),
            inertia: Inertia(10.0),
            ..default()
        },
        RigidBody::Dynamic,
    ));
}

fn tag_numbers_touching_symbols(
    // mut commands: Commands,
    symbols: Query<&CollidingEntities, With<Symbol>>,
    numbers: Query<&Number>,
) {
    let answer: usize = symbols
        .iter()
        .flat_map(|colliding_entities| colliding_entities.0.iter())
        .map(|entity| {
            numbers
                .get_component::<Number>(*entity)
                .map(|number| number.value)
        })
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .sum();
    println!("Answer: {}", answer);
}

#[derive(Event, Debug, Reflect)]
pub enum MovementAction {
    Move(Vec2),
    Stop,
}

fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time<Physics>>,
) {
    if time.is_paused() {
        return;
    }
    let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
    let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);
    let up = keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]);
    let down = keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]);
    let space = keyboard_input.pressed(KeyCode::Space);
    if space {
        movement_event_writer.send(MovementAction::Stop);
        return;
    }
    let horizontal = right as i8 - left as i8;
    let vertical = up as i8 - down as i8;
    let direction = Vec2::new(horizontal as Scalar, vertical as Scalar);
    if direction != Vec2::ZERO {
        movement_event_writer.send(MovementAction::Move(direction));
    }
}

fn has_movement(mut reader: EventReader<MovementAction>) -> bool {
    !reader.read().next().is_none()
}
fn movement(
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementAction>,
    mut controllers: Query<&mut LinearVelocity, With<Camera2d>>,
) {
    let delta_time = time.delta_seconds_f64().adjust_precision();
    let movement_acceleration = 2000.0;
    for event in movement_event_reader.read() {
        for mut linear_velocity in &mut controllers {
            match event {
                MovementAction::Stop => {
                    linear_velocity.x = 0.0;
                    linear_velocity.y = 0.0;
                }
                MovementAction::Move(direction) => {
                    linear_velocity.x += direction.x * movement_acceleration * delta_time;
                    linear_velocity.y += direction.y * movement_acceleration * delta_time;
                }
            }
        }
    }
}

fn apply_movement_damping(
    mut query: Query<
        (&mut LinearVelocity, &mut AngularVelocity),
        (With<Camera2d>, Without<Sleeping>),
    >,
    time: Res<Time<Physics>>,
) {
    if time.is_paused() {
        return;
    }
    let damping_factor = 0.95;
    for (mut linear_velocity, mut angular_velocity) in &mut query {
        linear_velocity.x *= damping_factor;
        if linear_velocity.x.abs() < 0.001 {
            linear_velocity.x = 0.0;
        }
        linear_velocity.y *= damping_factor;
        if linear_velocity.y.abs() < 0.001 {
            linear_velocity.y = 0.0;
        }
        angular_velocity.0 *= damping_factor;
        if angular_velocity.0.abs() < 0.001 {
            angular_velocity.0 = 0.0;
        }
    }
}

// fn main() {

//     let input = include_str!("./input.txt");
//     let result = time_function!(part1, input);
//     println!("Part 1: {}", result);
//     let result = time_function!(part2, input);
//     println!("Part 2: {}", result);
// }

// fn part1(input: &str) -> usize {
// }

// fn part2(input: &str) -> usize {
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // #[test]
//     // fn test_part1() {
//     //     let input = include_str!("./input.txt");
//     //     assert_eq!(part1(input), 12345);
//     // }

//     // #[test]
//     // fn test_part2() {
//     //     let input = include_str!("./input.txt");
//     //     assert_eq!(part2(input), 12345);
//     // }

//     #[test]
//     fn part1_example() {
//         let input = "";
//         assert_eq!(part1(input), 8);
//     }

//     // #[test]
//     // fn part2_example() {
//     //     let input = "";

//     //     let line_answers = [48, 12, 1560, 630, 36];
//     //     input.lines().zip(line_answers.iter()).for_each(|(line, answer)| {
//     //         assert_eq!(part2(line), *answer, "Line: {}", line);
//     //     });

//     //     assert_eq!(part2(input), 2286);
//     // }

//     // #[test]
//     // fn part1_badanswer() {
//     //     assert!(
//     //         part2(include_str!("./input.txt")) < 12345,
//     //         "Answer too high"
//     //     );
//     // }
//     // #[test]
//     // fn part2_badanswer() {
//     //     assert!(
//     //         part2(include_str!("./input.txt")) < 12345,
//     //         "Answer too high"
//     //     );
//     // }
// }
