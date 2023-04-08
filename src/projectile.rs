use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody, Velocity};

use crate::{
    collision::{HitBehaviour, HitBehaviours, HitBoxBundle, MyCollisionGroups},
    damage::DamageKind,
};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Default, Bundle)]
pub struct ProjectileBundle {
    sprite_bundle: SpriteBundle,
    rigid_body: RigidBody,
    velocity: Velocity,
}

pub fn spawn_projectile(
    mut commands: Commands,
    radius: f32,
    origin: Transform,
    velocity: Velocity,
    image: Handle<Image>,
) {
    let hit_box = commands
        .spawn(
            HitBoxBundle::default()
                .collider(Collider::ball(radius))
                .memberships(MyCollisionGroups::PLAYER)
                .filters(MyCollisionGroups::ENEMY),
        )
        .insert(HitBehaviours {
            hit_behaviours: vec![
                HitBehaviour::Damage {
                    affect_self: false,
                    amount: 10.0,
                    kind: DamageKind::Lethal,
                },
                HitBehaviour::Kill {
                    affect_self: true,
                    fade_time: 0.0,
                },
            ],
        })
        .id();

    commands
        .spawn(ProjectileBundle {
            sprite_bundle: SpriteBundle {
                transform: origin,
                texture: image,
                ..Default::default()
            },
            rigid_body: RigidBody::Dynamic,
            velocity,
        })
        .add_child(hit_box);
}
