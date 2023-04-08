use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody};

use crate::{
    collision::{HitBoxBundle, HitEvent, MyCollisionGroups, PhysicsCollisionBundle},
    damage::{DamageEvent, DamageKind},
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
        .spawn((
            SpatialBundle::default(),
            HitBoxBundle::default()
                .collider(Collider::ball(15.0))
                .memberships(MyCollisionGroups::ENEMY)
                .filters(MyCollisionGroups::PLAYER),
        ))
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

pub fn bomb_hit_behaviour(
    mut hit_events: EventReader<HitEvent>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for hit_event in hit_events.iter() {
        /* Self implode */
        let self_damage = DamageEvent {
            source: hit_event.source,
            target: hit_event.source,
            amount: 9999.9,
            kind: DamageKind::Lethal,
        };

        /* Damage the target */
        let target_damage = DamageEvent {
            source: hit_event.source,
            target: hit_event.target,
            amount: 5.0,
            kind: DamageKind::Lethal,
        };

        damage_events.send_batch([self_damage, target_damage]);
    }
}
