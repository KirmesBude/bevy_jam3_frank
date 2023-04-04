use bevy::prelude::*;

use crate::stats::base::Health;

#[derive(Debug, Component)]
pub struct DamageOnMove {
    pub damage: f32,
}

pub fn damage_on_move(query: Query<(&DamageOnMove, &mut Health)>) {
    
}