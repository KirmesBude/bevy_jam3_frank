use bevy::prelude::*;

use crate::player::spawn_player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_systems(
            (spawn_camera, apply_system_buffers)
                .chain()
                .before(spawn_player),
        )
        .add_system(update_cursor_entity);
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera_entity = commands.spawn(Camera2dBundle::default()).id();
    commands.spawn(CameraCursorBundle::with_camera_cursot(CameraCursor {
        camera_entity,
    }));
}

#[derive(Debug, Component)]
pub struct CameraCursor {
    pub camera_entity: Entity,
}

#[derive(Debug, Bundle)]
pub struct CameraCursorBundle {
    pub transform_bundle: TransformBundle,
    pub camera_cursor: CameraCursor,
}

impl CameraCursorBundle {
    pub fn with_camera_cursot(camera_cursor: CameraCursor) -> Self {
        Self {
            transform_bundle: Default::default(),
            camera_cursor,
        }
    }
}

fn update_cursor_entity(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut camera_cursor_query: Query<(&mut Transform, &CameraCursor)>,
) {
    for (mut camera_cursor_transform, camera_cursor) in camera_cursor_query.iter_mut() {
        if let Ok((camera, camera_transform)) = camera_query.get(camera_cursor.camera_entity) {
            if let Ok(window) = windows.get_single() {
                //TODO: Should be the window for this camera
                if let Some(cursor_position) = window.cursor_position() {
                    if let Some(cursor_world_position) = camera
                        .viewport_to_world(camera_transform, cursor_position)
                        .map(|ray| ray.origin)
                    {
                        camera_cursor_transform.translation = cursor_world_position;
                    }
                }
            }
        }
    }
}
