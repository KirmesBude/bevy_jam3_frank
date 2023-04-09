use bevy::prelude::*;

use crate::{damage::FlashColor, side_effects::debuffs::dead::Dead, stats::base::Health};

pub struct HealPlugin;

impl Plugin for HealPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HealEvent>();
    }
}

pub struct HealEvent {
    pub source: Entity,
    pub target: Entity,
    pub amount: f32,
}

fn apply_heal(
    mut target_health_query: Query<(&mut Health, Option<&mut FlashColor>), Without<Dead>>,
    mut heal_events: EventReader<HealEvent>,
) {
    for heal_event in heal_events.iter() {
        if let Ok((mut target_health, maybe_flash_color)) =
            target_health_query.get_mut(heal_event.target)
        {
            let new_health = target_health.current + heal_event.amount;
            let new_health = new_health.min(target_health.max);

            // "Flash" green
            if let Some(mut flash_color) = maybe_flash_color {
                flash_color.color.set_g(3.0);
                flash_color.timer = Timer::from_seconds(0.2, TimerMode::Once);
            }

            println!(
                "{:?} received {} heal from {:?}. New health: {}!",
                heal_event.target, heal_event.amount, heal_event.source, target_health.current
            );
        }
    }
}
