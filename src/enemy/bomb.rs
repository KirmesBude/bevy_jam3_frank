use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, LockedAxes, RigidBody};
use bevy_trickfilm::prelude::{AnimationClip2D, AnimationPlayer2D};

use crate::{
    collision::{
        HitBehaviour, HitBehaviours, HitBoxBundle, HurtBoxBundle, MyCollisionGroups,
        PhysicsCollisionBundle,
    },
    damage::DamageKind,
    movement::Follow,
    side_effects::debuffs::dead::{KillBehaviour, KillBehaviours},
    stats::base::{BaseStatsBundle, Health, MovementSpeed},
};

use super::{EnemyAssets, EnemyBundle};

#[derive(Debug, Default, Component)]
pub struct Bomb;

#[derive(Debug, Default)]
pub struct BombAnimations {
    pub idle: Handle<AnimationClip2D>,
    pub explode: Handle<AnimationClip2D>,
}

pub fn spawn_bomb(
    time: &Res<Time>,
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
                    override_source: None,
                    affect_self: false,
                    amount: 20.0,
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
            .collider(Collider::ball(10.0))
            .memberships(MyCollisionGroups::ENEMY),))
        .id();

    let mut animation_player = AnimationPlayer2D::default();
    animation_player.start(enemy_assets.bomb.idle.clone_weak());
    let animation_player = animation_player;
    let health = 30.0 + (((time.elapsed_seconds() / 30.0).floor() + 1.0) * 20.0);
    commands
        .spawn(EnemyBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                transform: transform.with_scale(Vec3::splat(2.0)),
                ..Default::default()
            },
            base_stats_bundle: BaseStatsBundle {
                health: Health {
                    current: health,
                    max: health,
                },
                movement_speed: MovementSpeed {
                    max: 20.0,
                    min: 10.0,
                    ..Default::default()
                },
            },
            physics_collision_bundle: PhysicsCollisionBundle::default()
                .collider(Collider::ball(15.0))
                .rigid_body(RigidBody::Dynamic),
            ..Default::default()
        })
        .insert(animation_player)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Bomb)
        .insert(Follow {
            entity: *player_entity,
        })
        .insert(KillBehaviours {
            kill_behaviours: vec![
                KillBehaviour::PlayAnimation {
                    affect_self: false,
                    animation_clip: enemy_assets.bomb.explode.clone_weak(),
                },
                KillBehaviour::SpawnHealDrop,
            ],
        })
        .push_children(&[hit_box, hurt_box]);
}
