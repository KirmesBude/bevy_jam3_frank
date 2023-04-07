use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::stats::base::MovementSpeed;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovedEvent>()
            .add_systems((look_at, follow, sync_position))
            .add_system(send_moved_event_and_update_position_ll.in_base_set(CoreSet::PostUpdate));
    }
}

pub struct MovedEvent {
    pub entity: Entity,
    pub distance: f32,
}

#[derive(Debug, Default, Component)]
pub struct PositionLL(pub Vec2);

impl PositionLL {
    pub fn from_transform(transform: &Transform) -> Self {
        Self(transform.translation.truncate())
    }
}

fn send_moved_event_and_update_position_ll(
    mut moved_events: EventWriter<MovedEvent>,
    mut query: Query<(Entity, &Transform, &mut PositionLL), Changed<Transform>>,
) {
    for (entity, transform, mut position_ll) in query.iter_mut() {
        let distance = transform.translation.truncate().distance(position_ll.0);

        if distance > 0.0 {
            moved_events.send(MovedEvent { entity, distance });
        }

        position_ll.0 = transform.translation.truncate();
    }
}

#[derive(Debug, Component)]
pub struct LookAt {
    pub entity: Entity,
}

pub fn look_at(
    mut query: Query<(&mut Transform, &LookAt)>,
    look_at_transforms: Query<&GlobalTransform>,
) {
    for (mut transform, look_at) in query.iter_mut() {
        if let Ok(look_at_transform) = look_at_transforms.get(look_at.entity) {
            let pos_looker = transform.translation.truncate();
            let pos_look_at = look_at_transform.translation().truncate();
            let pos = pos_look_at - pos_looker;
            let angle = pos.y.atan2(pos.x) + FRAC_PI_2;

            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}

#[derive(Debug, Component)]
pub struct Follow {
    pub entity: Entity,
}

pub fn follow(
    mut query: Query<(&mut Velocity, &GlobalTransform, &MovementSpeed, &Follow)>,
    follow_transforms: Query<&GlobalTransform>,
) {
    for (mut velocity, follower_transform, movement_speed, follow) in query.iter_mut() {
        if let Ok(follow_transform) = follow_transforms.get(follow.entity) {
            let direction = follow_transform.translation().truncate()
                - follower_transform.translation().truncate();
            velocity.linvel = direction.normalize_or_zero() * movement_speed.0;
        }
    }
}

#[derive(Debug, Component)]
pub struct SyncPosition {
    pub entity: Entity,
}

pub fn sync_position(
    mut query: Query<(&mut Transform, &SyncPosition)>,
    sync_transforms: Query<&GlobalTransform>,
) {
    for (mut syncer_transform, sync_position) in query.iter_mut() {
        if let Ok(sync_transform) = sync_transforms.get(sync_position.entity) {
            syncer_transform.translation.x = sync_transform.translation().x;
            syncer_transform.translation.y = sync_transform.translation().y;
        }
    }
}
