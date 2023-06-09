use bevy::prelude::*;
use bevy_trickfilm::prelude::{AnimationClip2D, AnimationPlayer2D};

use crate::heal::SpawnHealDropEvent;

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

#[derive(Debug, Default, Component)]
pub struct KillCounter(pub usize);

pub fn update_kill_counter(
    mut kill_counters: Query<&mut KillCounter>,
    mut kill_events: EventReader<KillEvent>,
) {
    for kill_event in kill_events.iter() {
        if let Ok(mut kill_counter) = kill_counters.get_mut(kill_event.source) {
            kill_counter.0 += 1;
        }
    }
}

#[derive(Debug, Clone, Component)]
pub struct KillBehaviours {
    pub kill_behaviours: Vec<KillBehaviour>,
}

#[derive(Debug, Clone)]
pub enum KillBehaviour {
    PlayAnimation {
        affect_self: bool,
        animation_clip: Handle<AnimationClip2D>,
    },
    SpawnHealDrop,
}

pub fn apply_kill_behaviour(
    mut kill_events: EventReader<KillEvent>,
    mut animation_players: Query<&mut AnimationPlayer2D>,
    mut spawn_drop_events: EventWriter<SpawnHealDropEvent>,
    kill_behaviours: Query<&KillBehaviours>,
) {
    for kill_event in kill_events.iter() {
        if let Ok(kill_behaviours) = kill_behaviours.get(kill_event.target) {
            for kill_behaviour in kill_behaviours.kill_behaviours.iter() {
                match kill_behaviour {
                    KillBehaviour::PlayAnimation {
                        affect_self,
                        animation_clip,
                    } => {
                        let target = if *affect_self {
                            kill_event.source
                        } else {
                            kill_event.target
                        };
                        if let Ok(mut animation_player) = animation_players.get_mut(target) {
                            animation_player.play(animation_clip.clone_weak());
                        }
                    }
                    KillBehaviour::SpawnHealDrop => {
                        spawn_drop_events.send(SpawnHealDropEvent {
                            entity: kill_event.target,
                        });
                    }
                }
            }
        }
    }
}
