use bevy::prelude::*;

#[derive(Default, Debug, Bundle)]
pub struct BaseStatsBundle {
    pub health: Health,
    pub movement_speed: MovementSpeed,
}

#[derive(Debug, Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
        }
    }
}

#[derive(Default, Debug, Component)]
pub struct MovementSpeed {
    pub current: f32,
    pub max: f32,
    pub min: f32,
}
