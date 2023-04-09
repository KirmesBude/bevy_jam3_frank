use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;

use crate::{
    collision::PhysicsCollisionBundle, damage::FlashColor, player::Player,
    stats::base::BaseStatsBundle,
};

use self::bomb::{spawn_bomb, BombAnimations};

pub mod bomb;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyAssets>()
            .add_event::<SpawnEnemyEvent>()
            .add_startup_system(load_enemy_assets)
            .add_system(spawn_enemy_continiously)
            .add_system(spawn_enemy.after(spawn_enemy_continiously));
    }
}

#[derive(Debug, Default, Component)]
struct Enemy;

#[derive(Resource, Default)]
pub struct EnemyAssets {
    bomb: BombAnimations,
}

#[derive(Default, Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    sprite_sheet_bundle: SpriteSheetBundle,
    base_stats_bundle: BaseStatsBundle,
    physics_collision_bundle: PhysicsCollisionBundle,
    flash_color: FlashColor,
}

pub fn load_enemy_assets(asset_server: Res<AssetServer>, mut enemy_assets: ResMut<EnemyAssets>) {
    enemy_assets.bomb = BombAnimations {
        idle: asset_server.load("enemy_bomb.trickfilm#idle"),
        explode: asset_server.load("enemy_bomb.trickfilm#explode"),
    };
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
    players: Query<Entity, With<Player>>,
) {
    if let Ok(player_entity) = players.get_single() {
        for spawn_enemy_event in spawn_enemy_events.iter() {
            let transform = spawn_enemy_event.transform;
            match spawn_enemy_event.kind {
                EnemyKind::Bomb => {
                    spawn_bomb(&mut commands, &enemy_assets, &transform, &player_entity)
                }
            }
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
    player_transform: Query<&GlobalTransform, With<Player>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        if let Ok(player_transform) = player_transform.get_single() {
            let radius = 300.0;
            let random_angle = time.elapsed_seconds() % (2.0 * PI);
            let (x, y) = (
                player_transform.translation().x + radius * random_angle.cos(),
                player_transform.translation().y + radius * random_angle.sin(),
            );
            spawn_enemy_events.send(SpawnEnemyEvent {
                kind: EnemyKind::Bomb,
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
            });
        }
    }
}
