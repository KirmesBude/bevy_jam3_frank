use bevy::prelude::*;

use crate::damage::{DamageEvent, DamageKind};

use crate::side_effects::PositionLL;

#[derive(Debug, Component)]
pub struct DamageOnMove {
    pub damage: f32,
}

#[derive(Debug, Bundle)]
pub struct DamageOnMoveBundle {
    pub damage_on_move: DamageOnMove,
    pub position_ll: PositionLL,
}

pub fn damage_on_move(
    mut query: Query<(Entity, &Transform, &PositionLL, &DamageOnMove)>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for (entity, transform, position_ll, damage_on_move) in query.iter_mut() {
        let vect = transform.translation.truncate() - position_ll.0;
        let pos_delta = vect.length();
        let amount = pos_delta * damage_on_move.damage;

        if amount > 0.0 {
            damage_events.send(DamageEvent {
                source: entity,
                target: entity,
                amount,
                kind: DamageKind::NonLethal,
            });
        }
    }
}
