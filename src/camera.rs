use bevy::prelude::*;

use crate::player::Player;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn follow_player(
    mut camera_transform: Query<&mut Transform, With<Camera2d>>,
    player_transform: Query<
        &GlobalTransform,
        (With<Player>, Without<Camera2d>, Changed<Transform>),
    >,
) {
    if let Ok(player_transform) = player_transform.get_single() {
        if let Ok(mut camera_transform) = camera_transform.get_single_mut() {
            camera_transform.translation = player_transform.translation();
        }
    }
}
