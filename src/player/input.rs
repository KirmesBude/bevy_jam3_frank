use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use crate::{movement::VelocityVector, stats::base::MovementSpeed};

use super::Player;

pub fn move_player(
    mut player_velocity_vector: Query<(&mut VelocityVector, &MovementSpeed), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok((mut player_velocity_vector, movement_speed)) =
        player_velocity_vector.get_single_mut()
    {
        let mut velocity_vector = Vec2::ZERO;
        if keyboard_input.pressed(KeyCode::Up) {
            velocity_vector += Vec2::Y;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            velocity_vector += Vec2::NEG_Y;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            velocity_vector += Vec2::X;
        }

        if keyboard_input.pressed(KeyCode::Left) {
            velocity_vector += Vec2::NEG_X;
        }

        player_velocity_vector.0 = velocity_vector.normalize_or_zero() * movement_speed.0;
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
