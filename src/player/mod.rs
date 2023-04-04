use bevy::prelude::*;

use crate::{
    side_effects::{
        debuffs::damage_on_move::{DamageOnMove, DamageOnMoveBundle},
        PositionLL,
    },
    stats::base::{BaseStatsBundle, Health, HurtBox, MovementSpeed},
};

pub mod input;

#[derive(Default, Component)]
pub struct Player;

#[derive(Resource, Default)]
pub struct PlayerAssets {
    sprite: Handle<Image>,
}

pub fn load_player_assets(asset_server: Res<AssetServer>, mut player_assets: ResMut<PlayerAssets>) {
    player_assets.sprite = asset_server.load("player.png");
}

pub fn spawn_player(mut commands: Commands, player_assets: Res<PlayerAssets>) {
    commands.spawn(PlayerBundle {
        sprite_bundle: SpriteBundle {
            texture: player_assets.sprite.clone_weak(),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..Default::default()
        },
        base_stats: BaseStatsBundle {
            health: Health(100.0),
            movement_speed: MovementSpeed(40.0),
            hurt_box: HurtBox(20.0),
        },
        damage_on_move_bundle: DamageOnMoveBundle {
            damage_on_move: DamageOnMove { damage: 0.5 },
            position_ll: PositionLL(Vec2::ZERO),
        },
        player: Player,
    });
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    sprite_bundle: SpriteBundle,
    base_stats: BaseStatsBundle,
    damage_on_move_bundle: DamageOnMoveBundle,
}
