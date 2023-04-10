use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody, Velocity};

use crate::{
    collision::{HitBehaviour, HitBehaviours, HitBoxBundle, MyCollisionGroups},
    damage::FlashColor,
    movement::Follow,
    side_effects::debuffs::dead::Dead,
    stats::base::{Health, MovementSpeed},
};

pub struct HealPlugin;

impl Plugin for HealPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HealEvent>()
            .add_event::<SpawnHealDropEvent>()
            .init_resource::<HealDropAssets>()
            .add_startup_system(load_heal_drop_assets)
            .add_systems((spawn_heal_drops, drops_in_range_follow, apply_heal));
    }
}

pub struct HealEvent {
    pub source: Entity,
    pub target: Entity,
    pub amount: f32,
}

fn apply_heal(
    mut target_health_query: Query<(&mut Health, Option<&mut FlashColor>), Without<Dead>>,
    mut heal_events: EventReader<HealEvent>,
) {
    for heal_event in heal_events.iter() {
        if let Ok((mut target_health, maybe_flash_color)) =
            target_health_query.get_mut(heal_event.target)
        {
            let new_health = target_health.current + heal_event.amount;
            let new_health = new_health.min(target_health.max);
            target_health.current = new_health;

            // "Flash" green
            if let Some(mut flash_color) = maybe_flash_color {
                let mut new_color = Color::default();
                new_color.set_g(3.0);
                flash_color.color = new_color;
                flash_color.timer = Timer::from_seconds(0.2, TimerMode::Once);
            }

            println!(
                "{:?} received {} heal from {:?}. New health: {}!",
                heal_event.target, heal_event.amount, heal_event.source, target_health.current
            );
        }
    }
}

#[derive(Debug, Default, Component)]
pub struct HealDrop;

#[derive(Default, Bundle)]
pub struct HealDropBundle {
    spite_bundle: SpriteBundle,
    heal_drop: HealDrop,
    movement_speed: MovementSpeed,
    rigid_body: RigidBody,
    velocity: Velocity,
}

#[derive(Debug, Default, Component)]
pub struct HealDropSearch {
    pub radius: f32,
}

fn drops_in_range_follow(
    mut commands: Commands,
    searchers: Query<(Entity, &GlobalTransform, &HealDropSearch), Without<Dead>>,
    drops: Query<(Entity, &GlobalTransform), (With<HealDrop>, Without<Dead>)>,
) {
    if let Ok((follow_entity, searcher_transform, search)) = searchers.get_single() {
        for (entity, transform) in drops.iter() {
            let distance = searcher_transform
                .translation()
                .truncate()
                .distance(transform.translation().truncate());

            if distance < search.radius {
                commands.entity(entity).insert(Follow {
                    entity: follow_entity,
                });
            }
        }
    }
}

#[derive(Debug, Default, Resource)]
pub struct HealDropAssets {
    heal_drop: Handle<Image>,
}

fn load_heal_drop_assets(
    asset_server: Res<AssetServer>,
    mut heal_drop_assets: ResMut<HealDropAssets>,
) {
    heal_drop_assets.heal_drop = asset_server.load("health_drop.png");
}

pub struct SpawnHealDropEvent {
    pub entity: Entity,
}

fn spawn_heal_drops(
    mut commands: Commands,
    transforms: Query<&Transform>,
    mut spawn_heal_drop_events: EventReader<SpawnHealDropEvent>,
    heal_drop_assets: Res<HealDropAssets>,
) {
    for event in spawn_heal_drop_events.iter() {
        if let Ok(transform) = transforms.get(event.entity) {
            let hit_box = commands
                .spawn((HitBoxBundle::default()
                    .collider(Collider::ball(2.0))
                    .memberships(MyCollisionGroups::HEAL_DROP)
                    .filters(MyCollisionGroups::PLAYER),))
                .insert(HitBehaviours {
                    hit_behaviours: vec![
                        HitBehaviour::Heal {
                            affect_self: false,
                            amount: 10.0,
                        },
                        HitBehaviour::Kill {
                            affect_self: true,
                            fade_time: 0.0,
                        },
                    ],
                })
                .id();

            commands
                .spawn(HealDropBundle {
                    spite_bundle: SpriteBundle {
                        texture: heal_drop_assets.heal_drop.clone_weak(),
                        transform: transform.with_scale(Vec3::splat(3.0)),
                        ..Default::default()
                    },
                    movement_speed: MovementSpeed {
                        current: 100.0,
                        max: 100.0,
                        min: 100.0,
                    },
                    rigid_body: RigidBody::Dynamic,
                    ..Default::default()
                })
                .add_child(hit_box);
        }
    }
}
