use bevy::prelude::*;

mod camera;
mod damage;
mod enemy;
mod movement;
mod player;
mod side_effects;
mod stats;
mod collision;

use bevy_rapier2d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use camera::CameraPlugin;
use damage::DamagePlugin;
use enemy::EnemyPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;
use side_effects::SideEffectsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(MovementPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(DamagePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(SideEffectsPlugin)
        .add_plugin(EnemyPlugin)
        .add_startup_system(rapier_setup)
        .run();
}

fn rapier_setup(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}
