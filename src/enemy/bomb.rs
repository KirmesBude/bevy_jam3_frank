use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody};

use crate::{
    collision::{
        HitBehaviour, HitBehaviours, HitBoxBundle, MyCollisionGroups, PhysicsCollisionBundle,
    },
    damage::DamageKind,
    movement::Follow,
    stats::base::{BaseStatsBundle, Health, MovementSpeed},
};

use super::{EnemyAssets, EnemyBundle};

#[derive(Debug, Default, Component)]
pub struct Bomb;

pub fn spawn_bomb(
    commands: &mut Commands,
    enemy_assets: &Res<EnemyAssets>,
    transform: &Transform,
    player_entity: &Entity,
) {
    let hit_box = commands
        .spawn((HitBoxBundle::default()
            .collider(Collider::ball(15.0))
            .memberships(MyCollisionGroups::ENEMY)
            .filters(MyCollisionGroups::PLAYER),))
        .insert(HitBehaviours {
            hit_behaviours: vec![
                HitBehaviour::Damage {
                    amount: 10.0,
                    kind: DamageKind::Lethal,
                },
                HitBehaviour::KillSelf { fade_time: 0.5 },
            ],
        })
        .id();
    commands
        .spawn(EnemyBundle {
            sprite_bundle: SpriteBundle {
                transform: transform.with_scale(Vec3::splat(2.0)),
                texture: enemy_assets.bomb.clone_weak(),
                ..Default::default()
            },
            base_stats_bundle: BaseStatsBundle {
                health: Health(50.0),
                movement_speed: MovementSpeed(20.0),
            },
            physics_collision_bundle: PhysicsCollisionBundle::default()
                .collider(Collider::ball(15.0))
                .rigid_body(RigidBody::Dynamic),
            ..Default::default()
        })
        .insert(Bomb)
        .insert(Follow {
            entity: *player_entity,
        })
        .add_child(hit_box);
}
