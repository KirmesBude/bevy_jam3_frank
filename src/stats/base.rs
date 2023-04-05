use bevy::prelude::*;

#[derive(Default, Debug, Bundle)]
pub struct BaseStatsBundle {
    pub health: Health,
    pub movement_speed: MovementSpeed,
    pub hurt_box: HurtBox,
}

#[derive(Debug, Component)]
pub struct Health(pub f32);

impl Default for Health {
    fn default() -> Self {
        Self(100.0)
    }
}

#[derive(Default, Debug, Component)]
pub struct MovementSpeed(pub f32);

#[derive(Default, Debug, Component)]
pub struct HurtBox(pub f32);
