use bevy::prelude::*;

mod camera;
mod player;
mod stats;
mod side_effects;

use camera::{follow_player, spawn_camera};
use player::{
    input::{look_at_cursor, move_player},
    load_player_assets, spawn_player, PlayerAssets,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<PlayerAssets>()
        .add_startup_system(load_player_assets)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_system(move_player)
        .add_system(follow_player)
        .add_system(look_at_cursor)
        .run();
}
