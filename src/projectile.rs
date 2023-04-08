use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody, Velocity};

use crate::{
    collision::{HitBehaviour, HitBehaviours, HitBoxBundle, MyCollisionGroups},
    damage::DamageKind,
    movement::LookAt,
    player::Player,
};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ProjectileAssets>()
            .add_startup_system(load_projectile_assets)
            .add_system(shoot);
    }
}

#[derive(Default, Bundle)]
pub struct ProjectileBundle {
    sprite_bundle: SpriteBundle,
    rigid_body: RigidBody,
    velocity: Velocity,
}

#[derive(Resource, Default)]
pub struct ProjectileAssets {
    pill: Handle<Image>,
}

pub fn load_projectile_assets(
    asset_server: Res<AssetServer>,
    mut projectile_assets: ResMut<ProjectileAssets>,
) {
    projectile_assets.pill = asset_server.load("pill.png");
}

fn shoot(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    player_query: Query<(&Transform, &LookAt), With<Player>>,
    transforms: Query<&GlobalTransform>,
    projectile_assets: ResMut<ProjectileAssets>,
) {
    if let Ok((player_transform, player_look_at)) = player_query.get_single() {
        if mouse_input.just_pressed(MouseButton::Left) {
            if let Ok(transform) = transforms.get(player_look_at.entity) {
                let origin = player_transform;
                let direction =
                    transform.translation().truncate() - player_transform.translation.truncate();
                let direction = direction.normalize();
                let velocity = Velocity {
                    linvel: direction * 100.0,
                    ..Default::default()
                };

                spawn_projectile(
                    &mut commands,
                    10.0,
                    origin,
                    velocity,
                    projectile_assets.pill.clone_weak(),
                );
            }
        }
    }
}

fn spawn_projectile(
    commands: &mut Commands,
    radius: f32,
    origin: &Transform,
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
                transform: *origin,
                texture: image,
                ..Default::default()
            },
            rigid_body: RigidBody::Dynamic,
            velocity,
        })
        .add_child(hit_box);
}
