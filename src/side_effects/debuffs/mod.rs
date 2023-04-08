use bevy::prelude::*;

use crate::damage::DamageSet;

use self::{
    damage_on_move::create_damage_on_move,
    dead::{apply_kill_event, remove_dead_entities, KillEvent},
};

pub mod damage_on_move;
pub mod dead;

pub struct DebuffPlugin;

impl Plugin for DebuffPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<KillEvent>()
            .add_system(create_damage_on_move.in_set(DamageSet::PreApply))
            .add_systems(
                (apply_kill_event, remove_dead_entities)
                    .chain()
                    .in_set(DamageSet::PostApply),
            );
    }
}
