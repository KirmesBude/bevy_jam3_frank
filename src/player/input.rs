use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use super::Player;

pub fn move_player(
    time: Res<Time>,
    mut player_transform: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok(mut player_transform) = player_transform.get_single_mut() {
        // TODO: fix so diagonal movement is not faster
        if keyboard_input.pressed(KeyCode::Up) {
            player_transform.translation.y += 40.0 * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::Down) {
            player_transform.translation.y -= 40.0 * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::Right) {
            player_transform.translation.x += 40.0 * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::Left) {
            player_transform.translation.x -= 40.0 * time.delta_seconds();
        }
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
