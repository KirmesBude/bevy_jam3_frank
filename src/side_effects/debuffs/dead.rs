use bevy::prelude::*;

#[derive(Debug, Default, Component)]
pub struct Dead {
    fade_timer: Timer,
}

impl Dead {
    pub fn with_fade_time(fade_time: f32) -> Self {
        Self {
            fade_timer: Timer::from_seconds(fade_time, TimerMode::Once),
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

pub struct KillEvent {
    source: Entity,
    target: Entity,
    fade_time: f32,
}

impl KillEvent {
    pub fn with_fade_time(source: Entity, target: Entity, fade_time: f32) -> Self {
        Self {
            source,
            target,
            fade_time,
        }
    }

    pub fn instant(source: Entity, target: Entity) -> Self {
        Self {
            source,
            target,
            fade_time: 0.0,
        }
    }
}

pub fn apply_kill_event(
    mut commands: Commands,
    mut kill_events: EventReader<KillEvent>,
    entities: Query<Entity>,
) {
    for kill_event in kill_events.iter() {
        if let Ok(entity) = entities.get(kill_event.target) {
            println!(
                "{:?} killed {:?} with fade_time {}!",
                kill_event.source, kill_event.target, kill_event.fade_time
            );

            commands
                .entity(entity)
                .insert(Dead::with_fade_time(kill_event.fade_time));
        }
    }
}
