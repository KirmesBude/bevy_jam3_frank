use bevy::prelude::*;
use bevy_rapier2d::{
    prelude::{CollisionEvent, Group, NoUserData, RapierConfiguration, RapierPhysicsPlugin},
    rapier::prelude::CollisionEventFlags,
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_startup_system(rapier_setup)
            .add_system(print_collision_events);
    }
}

pub struct CollisionMembership;

impl CollisionMembership {
    pub const PHYSICS: Group = Group::GROUP_1;
    pub const PLAYER: Group = Group::GROUP_2;
    pub const ENEMY: Group = Group::GROUP_3;
}

fn rapier_setup(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}

fn print_collision_events(mut collision_events: EventReader<CollisionEvent>) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity_l, entity_r, flags)
                if flags.contains(CollisionEventFlags::SENSOR) =>
            {
                println!("{:?} has started colliding with {:?}!", entity_l, entity_r)
            }
            CollisionEvent::Stopped(entity_l, entity_r, flags)
                if flags.contains(CollisionEventFlags::SENSOR) =>
            {
                println!("{:?} has stoppde colliding with {:?}!", entity_l, entity_r)
            }
            _ => {}
        }
    }
}
