use bevy::prelude::*;
use bevy_rapier2d::prelude::{ActiveEvents, Collider, CollisionGroups, RigidBody, Sensor};

use crate::{
    collision::CollisionMembership,
    stats::base::{BaseStatsBundle, Health, MovementSpeed},
};

use super::{Enemy, EnemyAssets, EnemyBundle};

#[derive(Debug, Default, Component)]
pub struct Bomb;

pub fn spawn_bomb(commands: &mut Commands, enemy_assets: &Res<EnemyAssets>, transform: &Transform) {
    let hit_box = commands
        .spawn((
            SpatialBundle::default(),
            Collider::ball(15.0),
            CollisionGroups::new(CollisionMembership::ENEMY, CollisionMembership::PLAYER),
            ActiveEvents::COLLISION_EVENTS,
            Sensor,
        ))
        .id();
    commands
        .spawn(EnemyBundle {
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
            collision_group: CollisionGroups::new(
                CollisionMembership::PHYSICS,
                CollisionMembership::PHYSICS,
            ),
            rigid_body: RigidBody::Dynamic,
            ..Default::default()
        })
        .insert(Bomb)
        .add_child(hit_box);
}
