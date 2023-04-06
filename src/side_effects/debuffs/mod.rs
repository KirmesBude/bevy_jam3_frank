use bevy::prelude::*;

use crate::{damage::DamageSet, movement::send_moved_event};

use self::{dead::apply_dead_from_health, damage_on_move::create_damage_on_move};

pub mod damage_on_move;
pub mod dead;

pub struct DebuffPlugin;

impl Plugin for DebuffPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_damage_on_move.in_set(DamageSet::PreApply).after(send_moved_event))
            .add_system(apply_dead_from_health.in_set(DamageSet::PostApply));
    }
}
