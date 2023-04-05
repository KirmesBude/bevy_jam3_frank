use bevy::prelude::*;

use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_system(follow_player);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn follow_player(
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
