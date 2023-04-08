use bevy::prelude::*;

use crate::stats::base::Health;

#[derive(Debug, Component)]
pub struct Dead {
    fade_timer: Timer,
}

pub fn apply_dead_from_health(
    mut commands: Commands,
    query: Query<(Entity, &Health), Without<Dead>>,
) {
    for (entity, health) in query.iter() {
        if health.0 <= 0.0 {
            commands.entity(entity).insert(Dead {
                fade_timer: Timer::from_seconds(2.0, TimerMode::Once),
            });
        }
    }
}

pub fn remove_dead_entities(
    mut commands: Commands,
    time: Res<Time>,
    mut dead_query: Query<(Entity, &mut Dead)>,
) {
    for (dead_entity, mut dead) in dead_query.iter_mut() {
        if dead.fade_timer.tick(time.delta()).just_finished() {
            commands.entity(dead_entity).despawn_descendants();
            commands.entity(dead_entity).despawn();
        }
    }
}
