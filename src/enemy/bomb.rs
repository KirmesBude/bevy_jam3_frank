use bevy::prelude::*;
use bevy_rapier2d::prelude::{RigidBody, Collider, ActiveEvents};

use crate::{stats::base::{BaseStatsBundle, Health, MovementSpeed}};

use super::{EnemyAssets, EnemyBundle, Enemy};

#[derive(Debug, Default, Component)]
pub struct Bomb;

pub fn spawn_bomb(commands: &mut Commands, enemy_assets: &Res<EnemyAssets>, transform: &Transform) {
    commands.spawn(EnemyBundle {
        enemy: Enemy,
        sprite_bundle: SpriteBundle {
            transform: transform.with_scale(Vec3::splat(2.0)),
            texture: enemy_assets.bomb.clone_weak(),
            ..Default::default()
        },
        base_stats_bundle: BaseStatsBundle {
            health: Health(50.0),
            movement_speed: MovementSpeed(20.0),
        },
        collider: Collider::ball(15.0),
        rigid_body: RigidBody::Dynamic,
        ..Default::default()
    })
    .insert(Bomb)
    .insert(ActiveEvents::COLLISION_EVENTS);
}
