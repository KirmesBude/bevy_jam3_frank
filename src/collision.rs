use bevy::prelude::*;
use bevy_rapier2d::{
    prelude::{
        ActiveEvents, Collider, CollisionEvent, CollisionGroups, Group, NoUserData,
        RapierConfiguration, RapierPhysicsPlugin, RigidBody, Sensor, Velocity,
    },
    rapier::prelude::CollisionEventFlags,
};

use crate::side_effects::debuffs::dead::Dead;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_startup_system(rapier_setup)
            .add_event::<HitEvent>()
            .add_system(hit_detection);
    }
}

#[derive(Debug, Bundle)]
pub struct PhysicsCollisionBundle {
    velocity: Velocity,
    rigid_body: RigidBody,
    collider: Collider,
    collision_groups: CollisionGroups,
}

impl Default for PhysicsCollisionBundle {
    fn default() -> Self {
        Self {
            velocity: Default::default(),
            rigid_body: Default::default(),
            collider: Default::default(),
            collision_groups: CollisionGroups {
                memberships: MyCollisionGroups::PHYSICS,
                filters: MyCollisionGroups::PHYSICS,
            },
        }
    }
}

impl PhysicsCollisionBundle {
    pub fn rigid_body(self, rigid_body: RigidBody) -> Self {
        Self {
            rigid_body,
            collider: self.collider,
            ..Default::default()
        }
    }

    pub fn collider(self, collider: Collider) -> Self {
        Self {
            rigid_body: self.rigid_body,
            collider,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Component)]
pub struct HitBox;

#[derive(Debug, Bundle)]
pub struct HitBoxBundle {
    hit_box: HitBox,
    collider: Collider,
    sensor: Sensor,
    active_events: ActiveEvents,
    collision_groups: CollisionGroups,
}

impl Default for HitBoxBundle {
    fn default() -> Self {
        Self {
            hit_box: Default::default(),
            collider: Default::default(),
            sensor: Default::default(),
            active_events: ActiveEvents::COLLISION_EVENTS,
            collision_groups: Default::default(),
        }
    }
}

impl HitBoxBundle {
    pub fn collider(self, collider: Collider) -> Self {
        Self {
            collider,
            collision_groups: self.collision_groups,
            ..Default::default()
        }
    }

    // Consider returning Result if physics
    pub fn memberships(self, memberships: Group) -> Self {
        Self {
            collider: self.collider,
            collision_groups: CollisionGroups::new(
                memberships & !MyCollisionGroups::PHYSICS,
                self.collision_groups.filters,
            ), // Never can be part of physics
            ..Default::default()
        }
    }

    // Consider returning Result if physics
    pub fn filters(self, filters: Group) -> Self {
        Self {
            collider: self.collider,
            collision_groups: CollisionGroups::new(
                self.collision_groups.memberships,
                filters & !MyCollisionGroups::PHYSICS,
            ), // Never can be part of physics
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Component)]
pub struct HurtBox;

#[derive(Debug, Bundle)]
pub struct HurtBoxBundle {
    hurt_box: HurtBox,
    collider: Collider,
    sensor: Sensor,
    collision_groups: CollisionGroups,
}

impl Default for HurtBoxBundle {
    fn default() -> Self {
        Self {
            hurt_box: Default::default(),
            collider: Default::default(),
            sensor: Default::default(),
            collision_groups: CollisionGroups::new(
                Group::NONE,
                Group::ALL & !MyCollisionGroups::PHYSICS,
            ), // Hurtbox alawys needs to be able to interact with everyting but physics
        }
    }
}

impl HurtBoxBundle {
    pub fn collider(self, collider: Collider) -> Self {
        Self {
            collider,
            collision_groups: self.collision_groups,
            ..Default::default()
        }
    }

    // Consider returning Result
    pub fn memberships(self, memberships: Group) -> Self {
        Self {
            collider: self.collider,
            collision_groups: CollisionGroups::new(
                memberships & !MyCollisionGroups::PHYSICS,
                self.collision_groups.filters,
            ), // Never can be part of physics
            ..Default::default()
        }
    }
}

pub struct MyCollisionGroups;

impl MyCollisionGroups {
    pub const PHYSICS: Group = Group::GROUP_1;
    pub const PLAYER: Group = Group::GROUP_2;
    pub const ENEMY: Group = Group::GROUP_3;
}

fn rapier_setup(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}

pub struct HitEvent {
    pub source: Entity,
    pub target: Entity,
}

fn hit_detection(
    mut hit_events: EventWriter<HitEvent>,
    mut collision_events: EventReader<CollisionEvent>,
    hit_boxes: Query<&Parent, With<HitBox>>,
    hurt_boxes: Query<&Parent, With<HurtBox>>,
    not_dead: Query<(), Without<Dead>>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity_l, entity_r, flags)
                if flags.contains(CollisionEventFlags::SENSOR) =>
            {
                for (maybe_hit, maybe_hurt) in [(entity_l, entity_r), (entity_r, entity_l)] {
                    if let Ok(hit_parent) = hit_boxes.get(*maybe_hit) {
                        if let Ok(hurt_parent) = hurt_boxes.get(*maybe_hurt) {
                            if let (Ok(()), Ok(())) = (
                                not_dead.get(hurt_parent.get()),
                                not_dead.get(hit_parent.get()),
                            ) {
                                println!("{:?} hit {:?}", hit_parent.get(), hurt_parent.get());

                                hit_events.send(HitEvent {
                                    source: hit_parent.get(),
                                    target: hurt_parent.get(),
                                });
                            }
                        }
                    }
                }
            }
            /*
            CollisionEvent::Stopped(entity_l, entity_r, flags)
                if flags.contains(CollisionEventFlags::SENSOR) =>
            {
                println!("{:?} has stoppde colliding with {:?}!", entity_l, entity_r)
            }
            */
            _ => {}
        }
    }
}
