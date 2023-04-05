use bevy::prelude::*;

use crate::{movement::VelocityVector, stats::base::BaseStatsBundle};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyAssets>()
            .add_system(spawn_enemy)
            .add_system(follow_player);
    }
}

#[derive(Debug, Default, Component)]
struct Enemy;

#[derive(Resource, Default)]
pub struct EnemyAssets {
    simple: Handle<Image>,
}

#[derive(Default, Bundle)]
pub struct EnemyPlayer {
    enemy: Enemy,
    sprite_bundle: SpriteBundle,
    base_stats_bundle: BaseStatsBundle,
    velocity_vector: VelocityVector,
}

fn spawn_enemy() {}

fn follow_player() {}
