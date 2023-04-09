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
            .add_systems(
                (apply_damage, process_flash_color)
                    .chain()
                    .in_set(DamageSet::Apply),
            );
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

#[derive(Debug, Default, Component)]
pub struct FlashColor {
    color: Color,
    timer: Timer,
}

fn apply_damage(
    mut target_health_query: Query<(&mut Health, Option<&mut FlashColor>), Without<Dead>>,
    mut damage_events: EventReader<DamageEvent>,
    mut kill_events: EventWriter<KillEvent>,
) {
    for damage_event in damage_events.iter() {
        if let Ok((mut target_health, maybe_flash_color)) =
            target_health_query.get_mut(damage_event.target)
        {
            let new_health = target_health.0 - damage_event.amount;
            target_health.0 = match damage_event.kind {
                DamageKind::NonLethal => new_health.max(1.0),
                DamageKind::Lethal => new_health.max(0.0),
            };

            // "Flash" red
            if let Some(mut flash_color) = maybe_flash_color {
                flash_color.color.set_r(3.0);
                flash_color.timer = Timer::from_seconds(0.2, TimerMode::Once);
            }

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

fn process_flash_color(time: Res<Time>, mut query: Query<(&mut FlashColor, &mut Sprite)>) {
    for (mut flash_color, mut sprite) in query.iter_mut() {
        if flash_color.timer.tick(time.delta()).just_finished() {
            flash_color.color = Color::default();
        }

        sprite.color = flash_color.color;
    }
}
