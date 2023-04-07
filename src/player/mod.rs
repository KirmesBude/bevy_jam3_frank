use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, CollisionGroups, RigidBody, Sensor, Velocity};

use crate::{
    collision::CollisionMembership,
    movement::PositionLL,
    side_effects::debuffs::damage_on_move::{DamageOnMove, DamageOnMoveBundle},
    stats::base::{BaseStatsBundle, Health, MovementSpeed},
};

use self::input::{look_at_cursor, move_player};

mod input;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerAssets>()
            .add_startup_system(load_player_assets)
            .add_startup_system(spawn_player)
            .add_system(move_player)
            .add_system(look_at_cursor);
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
    velocity: Velocity,
    collision_groups: CollisionGroups,
    rigid_body: RigidBody,
    collider: Collider,
}

#[derive(Resource, Default)]
pub struct PlayerAssets {
    sprite: Handle<Image>,
}

fn load_player_assets(asset_server: Res<AssetServer>, mut player_assets: ResMut<PlayerAssets>) {
    player_assets.sprite = asset_server.load("player.png");
}

fn spawn_player(mut commands: Commands, player_assets: Res<PlayerAssets>) {
    let hurt_box = commands
        .spawn((
            SpatialBundle::default(),
            Collider::ball(15.0),
            CollisionGroups::new(CollisionMembership::PLAYER, CollisionMembership::ENEMY),
            Sensor,
        ))
        .id();
    let transform = Transform::from_scale(Vec3::splat(3.0));
    commands
        .spawn(PlayerBundle {
            sprite_bundle: SpriteBundle {
                texture: player_assets.sprite.clone_weak(),
                transform: transform,
                ..Default::default()
            },
            base_stats_bundle: BaseStatsBundle {
                health: Health(100.0),
                movement_speed: MovementSpeed(40.0),
            },
            damage_on_move_bundle: DamageOnMoveBundle {
                damage_on_move: DamageOnMove(0.5),
                position_ll: PositionLL::from_transform(&transform),
            },
            collider: Collider::ball(15.0),
            collision_groups: CollisionGroups::new(
                CollisionMembership::PHYSICS,
                CollisionMembership::PHYSICS,
            ),
            rigid_body: RigidBody::Dynamic,
            ..Default::default()
        })
        .add_child(hurt_box);
}
