use bevy::prelude::*;

use self::{buffs::BuffPlugin, debuffs::DebuffPlugin};

pub mod buffs;
pub mod debuffs;

pub struct SideEffectsPlugin;

impl Plugin for SideEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BuffPlugin).add_plugin(DebuffPlugin);
    }
}
