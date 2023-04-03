use bevy::prelude::*;

pub mod input;

#[derive(Component)]
pub struct Player;

#[derive(Resource, Default)]
pub struct PlayerAssets {
    sprite: Handle<Image>,
}

pub fn load_player_assets(asset_server: Res<AssetServer>, mut player_assets: ResMut<PlayerAssets>) {
    player_assets.sprite = asset_server.load("player.png");
}

pub fn spawn_player(mut commands: Commands, player_assets: Res<PlayerAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: player_assets.sprite.clone_weak(),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..Default::default()
        })
        .insert(Player);
}
