use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody, Velocity};

use crate::{
    player::Player,
    stats::base::{BaseStatsBundle, Health, HurtBox, MovementSpeed},
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyAssets>()
            .add_event::<SpawnEnemyEvent>()
            .add_startup_system(load_enemy_assets)
            .add_system(spawn_enemy_continiously)
            .add_system(spawn_enemy.after(spawn_enemy_continiously))
            .add_system(follow_player);
    }
}

#[derive(Debug, Default, Component)]
struct Enemy;

#[derive(Resource, Default)]
pub struct EnemyAssets {
    bomb: Handle<Image>,
}

#[derive(Default, Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    sprite_bundle: SpriteBundle,
    base_stats_bundle: BaseStatsBundle,
    velocity: Velocity,
    rigid_body: RigidBody,
    collider: Collider,
}

pub fn load_enemy_assets(asset_server: Res<AssetServer>, mut enemy_assets: ResMut<EnemyAssets>) {
    enemy_assets.bomb = asset_server.load("enemy_bomb.png");
}

pub enum EnemyKind {
    Bomb,
}

pub struct SpawnEnemyEvent {
    kind: EnemyKind,
    transform: Transform,
    // bundle: Box<dyn Bundle>,
}

fn spawn_enemy(
    mut commands: Commands,
    mut spawn_enemy_events: EventReader<SpawnEnemyEvent>,
    enemy_assets: Res<EnemyAssets>,
) {
    for spawn_enemy_event in spawn_enemy_events.iter() {
        let transform = spawn_enemy_event.transform;
        match spawn_enemy_event.kind {
            EnemyKind::Bomb => spawn_bomb(&mut commands, &enemy_assets, &transform),
        }
    }
}

fn spawn_bomb(commands: &mut Commands, enemy_assets: &Res<EnemyAssets>, transform: &Transform) {
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
            hurt_box: HurtBox(20.0),
        },
        collider: Collider::ball(15.0),
        rigid_body: RigidBody::Dynamic,
        ..Default::default()
    });
}

fn follow_player(
    mut enemy_query: Query<(&Transform, &MovementSpeed, &mut Velocity), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (enemy_transform, movement_speed, mut velocity) in enemy_query.iter_mut() {
            let direction =
                player_transform.translation.truncate() - enemy_transform.translation.truncate();
            velocity.linvel = direction.normalize_or_zero() * movement_speed.0;
        }
    }
}

struct EnemySpawnTimer(Timer);

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self(Timer::new(
            Duration::from_secs_f32(10.0),
            TimerMode::Repeating,
        ))
    }
}

fn spawn_enemy_continiously(
    time: Res<Time>,
    mut timer: Local<EnemySpawnTimer>,
    mut spawn_enemy_events: EventWriter<SpawnEnemyEvent>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        spawn_enemy_events.send(SpawnEnemyEvent {
            kind: EnemyKind::Bomb,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        });
    }
}
