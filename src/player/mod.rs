use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, LockedAxes, RigidBody};

use crate::{
    camera::CameraCursor,
    collision::{HurtBoxBundle, MyCollisionGroups, PhysicsCollisionBundle},
    damage::FlashColor,
    heal::HealDropSearch,
    movement::{LookAt, PositionLL, SyncPosition},
    side_effects::debuffs::{
        damage_on_move::{DamageOnMove, DamageOnMoveBundle},
        dead::KillCounter,
    },
    stats::base::{BaseStatsBundle, Health, MovementSpeed},
};

use self::input::move_player;

mod input;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerAssets>()
            .add_startup_system(load_player_assets)
            .add_startup_system(spawn_player.after(load_player_assets))
            .add_system(move_player);
    }
}

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, Bundle)]
pub struct PlayerBundle {
    player: Player,
    sprite_bundle: SpriteBundle,
    base_stats_bundle: BaseStatsBundle,
    damage_on_move_bundle: DamageOnMoveBundle,
    physics_collision_bundle: PhysicsCollisionBundle,
    flash_color: FlashColor,
    kill_counter: KillCounter,
    attack_rate: AttackRate,
    heal_drop_search: HealDropSearch,
}

#[derive(Debug, Default, Component)]
pub struct AttackRate {
    pub rate: f32,
    pub timer: Timer,
}

#[derive(Resource, Default)]
pub struct PlayerAssets {
    sprite: Handle<Image>,
}

fn load_player_assets(asset_server: Res<AssetServer>, mut player_assets: ResMut<PlayerAssets>) {
    player_assets.sprite = asset_server.load("player.png");
}

pub fn spawn_player(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    cameras: Query<Entity, With<Camera>>,
    camera_cursors: Query<Entity, With<CameraCursor>>,
) {
    let hurt_box = commands
        .spawn((HurtBoxBundle::default()
            .collider(Collider::ball(15.0))
            .memberships(MyCollisionGroups::PLAYER),))
        .id();

    let mut transform = Transform::from_scale(Vec3::splat(3.0));
    transform.translation.z = 1.0;
    let player_entity = commands
        .spawn(PlayerBundle {
            sprite_bundle: SpriteBundle {
                texture: player_assets.sprite.clone_weak(),
                transform,
                ..Default::default()
            },
            base_stats_bundle: BaseStatsBundle {
                health: Health {
                    max: 100.0,
                    current: 100.0,
                },
                movement_speed: MovementSpeed {
                    max: 40.0,
                    min: 5.0,
                    ..Default::default()
                },
            },
            damage_on_move_bundle: DamageOnMoveBundle {
                damage_on_move: DamageOnMove(0.5),
                position_ll: PositionLL::from_transform(&transform),
            },
            physics_collision_bundle: PhysicsCollisionBundle::default()
                .collider(Collider::ball(15.0))
                .rigid_body(RigidBody::Dynamic),
            attack_rate: AttackRate {
                rate: 0.4,
                ..Default::default()
            },
            heal_drop_search: HealDropSearch { radius: 200.0 },
            ..Default::default()
        })
        .insert(LockedAxes::ROTATION_LOCKED)
        .add_child(hurt_box)
        .id();

    if let Ok(camera_entity) = cameras.get_single() {
        commands.entity(camera_entity).insert(SyncPosition {
            entity: player_entity,
        });
    }

    if let Ok(camera_cursor_entity) = camera_cursors.get_single() {
        commands.entity(player_entity).insert(LookAt {
            entity: camera_cursor_entity,
        });
    }
}
