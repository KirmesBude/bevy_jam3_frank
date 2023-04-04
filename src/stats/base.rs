use bevy::prelude::*;

#[derive(Debug, Bundle)]
pub struct BaseStatsBundle {
    pub health: Health,
    pub movement_speed: MovementSpeed,
    pub hurt_box: HurtBox,
}

#[derive(Debug, Component)]
pub struct Health(pub f32);

#[derive(Debug, Component)]
pub struct MovementSpeed(pub f32);

#[derive(Debug, Component)]
pub struct HurtBox(pub f32);
