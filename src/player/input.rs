use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::stats::base::MovementSpeed;

use super::Player;

pub fn move_player(
    mut player_velocity_vector: Query<(&mut Velocity, &MovementSpeed), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok((mut player_velocity, movement_speed)) =
        player_velocity_vector.get_single_mut()
    {
        let up = keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]);
        let down = keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]);
        let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
        let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);
        if move_delta != Vec2::ZERO {
            move_delta /= move_delta.length();
        }

        player_velocity.linvel = move_delta * movement_speed.0;
    }
}

pub fn look_at_cursor(
    mut player_transform: Query<&mut Transform, With<Player>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    window: Query<&Window>,
) {
    if let Ok((camera, camera_transform)) = camera_query.get_single() {
        if let Ok(window) = window.get_single() {
            if let Some(cursor_position) = window.cursor_position() {
                if let Ok(mut player_transform) = player_transform.get_single_mut() {
                    let screen_position = camera
                        .world_to_viewport(camera_transform, player_transform.translation)
                        .unwrap();
                    let pos = cursor_position - screen_position;
                    let angle = pos.y.atan2(pos.x) + FRAC_PI_2;
                    player_transform.rotation = Quat::from_rotation_z(angle);
                }
            }
        }
    }
}
