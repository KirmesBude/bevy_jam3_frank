use bevy::prelude::*;

use crate::stats::base::Health;

#[derive(Debug, Component)]
struct Dead {
    fade_time: f32,
}

pub fn apply_dead_from_health(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in query.iter() {
        if health.0 <= 0.0 {
            commands.entity(entity).insert(Dead { fade_time: 2.0 });
        }
    }
}
