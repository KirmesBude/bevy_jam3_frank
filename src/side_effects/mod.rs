use bevy::prelude::*;

pub mod buffs;
pub mod debuffs;

#[derive(Debug, Component)]
pub struct PositionLL(pub Vec2);

// This should be placed in PostUpdate
pub fn save_ll_position(mut query: Query<(&Transform, &mut PositionLL)>) {
    for (tranform, mut position_ll) in query.iter_mut() {
        position_ll.0 = tranform.translation.truncate();
    }
}
