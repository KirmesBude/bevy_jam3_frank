use bevy::prelude::*;

use crate::{
    movement::{MovementSet, VelocityVector},
    side_effects::debuffs::damage_on_move::DamageOnMove,
    stats::base::{BaseStatsBundle, Health, HurtBox, MovementSpeed},
};

use self::input::{look_at_cursor, move_player};

mod input;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerAssets>()
            .add_startup_system(load_player_assets)
            .add_startup_system(spawn_player)
            .add_system(move_player.in_set(MovementSet::Update))
            .add_system(look_at_cursor.in_set(MovementSet::Update));
    }
}

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, Bundle)]
pub struct PlayerBundle {
    player: Player,
    sprite_bundle: SpriteBundle,
    base_stats_bundle: BaseStatsBundle,
    damage_on_move: DamageOnMove,
    velocity_vector: VelocityVector,
}

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
        base_stats_bundle: BaseStatsBundle {
            health: Health(100.0),
            movement_speed: MovementSpeed(40.0),
            hurt_box: HurtBox(20.0),
        },
        damage_on_move: DamageOnMove(0.5),
        ..Default::default()
    });
}
