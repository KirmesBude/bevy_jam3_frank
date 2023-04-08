use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::{side_effects::debuffs::dead::Dead, stats::base::MovementSpeed};

use super::Player;

pub type PlayerVelocity<'a> = (&'a mut Velocity, &'a MovementSpeed);

pub fn move_player(
    mut player_velocity_vector: Query<PlayerVelocity, (With<Player>, Without<Dead>)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok((mut player_velocity, movement_speed)) = player_velocity_vector.get_single_mut() {
        let up = keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]);
        let down = keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]);
        let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
        let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);
        if move_delta != Vec2::ZERO {
            move_delta /= move_delta.length();
        }

        player_velocity.linvel = move_delta * movement_speed.0;
    }
}
