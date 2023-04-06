use bevy::prelude::*;

use crate::damage::DamageSet;

use self::{damage_on_move::create_damage_on_move, dead::apply_dead_from_health};

pub mod damage_on_move;
pub mod dead;

pub struct DebuffPlugin;

impl Plugin for DebuffPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_damage_on_move.in_set(DamageSet::PreApply))
            .add_system(apply_dead_from_health.in_set(DamageSet::PostApply));
    }
}
