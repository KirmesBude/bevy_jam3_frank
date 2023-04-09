use bevy::prelude::*;

mod camera;
mod collision;
mod damage;
mod enemy;
mod movement;
mod player;
mod projectile;
mod side_effects;
mod stats;
mod ui;

use camera::CameraPlugin;
use collision::CollisionPlugin;
use damage::DamagePlugin;
use enemy::EnemyPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;
use projectile::ProjectilePlugin;
use side_effects::SideEffectsPlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(CollisionPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(DamagePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(SideEffectsPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(ProjectilePlugin)
        .add_plugin(UiPlugin)
        .run();
}
