use bevy::prelude::*;

use crate::{
    damage::{DamageEvent, DamageKind}, movement::{MovedEvent, PositionLL},
};

#[derive(Debug, Default, Bundle)]
pub struct DamageOnMoveBundle {
    pub damage_on_move: DamageOnMove,
    pub position_ll: PositionLL,
}

#[derive(Default, Debug, Component)]
pub struct DamageOnMove(pub f32);

pub fn create_damage_on_move(
    query: Query<&DamageOnMove>,
    mut moved_events: EventReader<MovedEvent>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for moved_event in moved_events.iter() {
        let entity = moved_event.entity;
        if let Ok(damage_on_move) = query.get(entity) {
            let amount = moved_event.distance * damage_on_move.0;
            damage_events.send(DamageEvent {
                source: entity,
                target: entity,
                amount,
                kind: DamageKind::NonLethal,
            });
        }
    }
}
