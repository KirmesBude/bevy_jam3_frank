use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, LockedAxes, RigidBody};

use crate::{
    collision::{
        HitBehaviour, HitBehaviours, HitBoxBundle, HurtBoxBundle, MyCollisionGroups,
        PhysicsCollisionBundle,
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
                    affect_self: false,
                    amount: 10.0,
                    kind: DamageKind::Lethal,
                },
                HitBehaviour::Kill {
                    affect_self: true,
                    fade_time: 0.5,
                },
            ],
        })
        .id();

    let hurt_box = commands
        .spawn((HurtBoxBundle::default()
            .collider(Collider::ball(15.0))
            .memberships(MyCollisionGroups::ENEMY),))
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
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Bomb)
        .insert(Follow {
            entity: *player_entity,
        })
        .push_children(&[hit_box, hurt_box]);
}
