use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody, Velocity};

use crate::{
    collision::{HitBehaviour, HitBehaviours, HitBoxBundle, MyCollisionGroups},
    damage::DamageKind,
    movement::LookAt,
    player::{AttackRate, Player},
    side_effects::debuffs::dead::KillEvent,
};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ProjectileAssets>()
            .add_startup_system(load_projectile_assets)
            .add_system(shoot)
            .add_system(projectile_lifetime);
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
    time: Res<Time>,
    mouse_input: Res<Input<MouseButton>>,
    mut player_query: Query<(Entity, &mut AttackRate, &Transform, &LookAt), With<Player>>,
    transforms: Query<&GlobalTransform>,
    projectile_assets: ResMut<ProjectileAssets>,
) {
    if let Ok((player_entity, mut attack_rate, player_transform, player_look_at)) =
        player_query.get_single_mut()
    {
        if attack_rate.timer.tick(time.delta()).finished() && mouse_input.pressed(MouseButton::Left)
        {
            if let Ok(transform) = transforms.get(player_look_at.entity) {
                let origin = player_transform;
                let direction =
                    transform.translation().truncate() - player_transform.translation.truncate();
                let direction = direction.normalize();
                let velocity = Velocity {
                    linvel: direction * 100.0,
                    angvel: 15.0,
                };

                spawn_projectile(
                    &mut commands,
                    10.0,
                    origin,
                    velocity,
                    projectile_assets.pill.clone_weak(),
                    player_entity,
                );
            }

            attack_rate.timer = Timer::from_seconds(attack_rate.rate, TimerMode::Once);
        }
    }
}

fn spawn_projectile(
    commands: &mut Commands,
    radius: f32,
    origin: &Transform,
    velocity: Velocity,
    image: Handle<Image>,
    shooter_entity: Entity,
) {
    let hit_box = commands
        .spawn(
            HitBoxBundle::default()
                .collider(Collider::ball(radius))
                .memberships(MyCollisionGroups::PLAYER_PROJECTILE)
                .filters(MyCollisionGroups::ENEMY),
        )
        .insert(HitBehaviours {
            hit_behaviours: vec![
                HitBehaviour::Damage {
                    override_source: Some(shooter_entity),
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

    let transform = Transform::from_xyz(origin.translation.x, origin.translation.y, 2.0)
        .with_scale(Vec3::splat(3.0));
    commands
        .spawn(ProjectileBundle {
            sprite_bundle: SpriteBundle {
                transform,
                texture: image,
                ..Default::default()
            },
            rigid_body: RigidBody::Dynamic,
            velocity,
        })
        .insert(Lifetime {
            timer: Timer::from_seconds(10.0, TimerMode::Once),
        })
        .add_child(hit_box);
}

#[derive(Debug, Default, Component)]
pub struct Lifetime {
    timer: Timer,
}

fn projectile_lifetime(
    time: Res<Time>,
    mut lifetimes: Query<(Entity, &mut Lifetime)>,
    mut kill_events: EventWriter<KillEvent>,
) {
    for (entity, mut lifetime) in lifetimes.iter_mut() {
        if lifetime.timer.tick(time.delta()).just_finished() {
            kill_events.send(KillEvent::instant(entity, entity));
        }
    }
}
