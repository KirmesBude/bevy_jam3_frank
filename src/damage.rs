use bevy::prelude::*;

use crate::{
    side_effects::debuffs::dead::{Dead, KillEvent},
    stats::base::Health,
};

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>()
            .configure_sets((DamageSet::PreApply, DamageSet::Apply, DamageSet::PostApply).chain())
            .add_system(apply_damage.in_set(DamageSet::Apply));
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum DamageSet {
    PreApply,
    Apply,
    PostApply,
}

pub struct DamageEvent {
    pub source: Entity,
    pub target: Entity,
    pub amount: f32,
    pub kind: DamageKind,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum DamageKind {
    NonLethal,
    #[default]
    Lethal,
}

fn apply_damage(
    mut target_health_query: Query<&mut Health, Without<Dead>>,
    mut damage_events: EventReader<DamageEvent>,
    mut kill_events: EventWriter<KillEvent>,
) {
    for damage_event in damage_events.iter() {
        if let Ok(mut target_health) = target_health_query.get_mut(damage_event.target) {
            let new_health = target_health.0 - damage_event.amount;
            target_health.0 = match damage_event.kind {
                DamageKind::NonLethal => new_health.max(1.0),
                DamageKind::Lethal => new_health.max(0.0),
            };

            println!(
                "{:?} received {} damage from {:?}. New health: {}!",
                damage_event.target, damage_event.amount, damage_event.source, target_health.0
            );

            if new_health <= 0.0 {
                kill_events.send(KillEvent::with_fade_time(
                    damage_event.source,
                    damage_event.target,
                    2.0,
                ));
            }
        }
    }
}
