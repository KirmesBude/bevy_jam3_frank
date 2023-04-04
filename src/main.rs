use bevy::prelude::*;

mod camera;
mod damage;
mod player;
mod side_effects;
mod stats;

use camera::{follow_player, spawn_camera};
use damage::{apply_damage, DamageEvent};
use player::{
    input::{look_at_cursor, move_player},
    load_player_assets, spawn_player, PlayerAssets,
};
use side_effects::{
    debuffs::{damage_on_move::damage_on_move, dead::apply_dead_from_health},
    save_ll_position,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<PlayerAssets>()
        .add_event::<DamageEvent>()
        .add_startup_system(load_player_assets)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_system(move_player)
        .add_system(follow_player)
        .add_system(look_at_cursor)
        .add_system(save_ll_position.before(move_player))
        .add_system(damage_on_move.after(move_player))
        .add_system(apply_damage.after(damage_on_move))
        .add_system(apply_dead_from_health.after(apply_damage))
        .run();
}
