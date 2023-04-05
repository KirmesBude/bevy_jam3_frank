use bevy::prelude::*;

mod camera;
mod damage;
mod enemy;
mod movement;
mod player;
mod side_effects;
mod stats;

use camera::CameraPlugin;
use damage::DamagePlugin;
use enemy::EnemyPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;
use side_effects::SideEffectsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MovementPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(DamagePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(SideEffectsPlugin)
        .add_plugin(EnemyPlugin)
        .run();
}
