use bevy::prelude::*;
use bevy_rapier2d::{
    prelude::{
        ActiveEvents, Collider, CollisionEvent, CollisionGroups, Group, NoUserData,
        RapierConfiguration, RapierPhysicsPlugin, RigidBody, Sensor, Velocity,
    },
    rapier::prelude::CollisionEventFlags,
};

use crate::{
    damage::{DamageEvent, DamageKind},
    heal::HealEvent,
    side_effects::debuffs::dead::{Dead, KillEvent},
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_startup_system(rapier_setup)
            .add_event::<HitEvent>()
            .add_systems((hit_detection, apply_hit_behaviour).chain());
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
    transform_bundle: TransformBundle,
    collider: Collider,
    sensor: Sensor,
    active_events: ActiveEvents,
    collision_groups: CollisionGroups,
}

impl Default for HitBoxBundle {
    fn default() -> Self {
        Self {
            hit_box: Default::default(),
            transform_bundle: Default::default(),
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
    transform_bundle: TransformBundle,
    collider: Collider,
    sensor: Sensor,
    collision_groups: CollisionGroups,
}

impl Default for HurtBoxBundle {
    fn default() -> Self {
        Self {
            hurt_box: Default::default(),
            transform_bundle: Default::default(),
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
    pub const PLAYER_PROJECTILE: Group = Group::GROUP_4;
    pub const HEAL_DROP: Group = Group::GROUP_5;
}

fn rapier_setup(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}

#[derive(Debug, Clone, Component)]
pub struct HitBehaviours {
    pub hit_behaviours: Vec<HitBehaviour>,
}

#[derive(Debug, Clone)]
pub enum HitBehaviour {
    Damage {
        override_source: Option<Entity>,
        affect_self: bool,
        amount: f32,
        kind: DamageKind,
    },
    Kill {
        affect_self: bool,
        fade_time: f32,
    },
    Heal {
        affect_self: bool,
        amount: f32,
    },
}

pub struct HitEvent {
    pub source: Entity,
    pub target: Entity,
    pub hit_behaviours: Option<HitBehaviours>,
}

fn hit_detection(
    mut hit_events: EventWriter<HitEvent>,
    mut collision_events: EventReader<CollisionEvent>,
    hit_boxes: Query<(&Parent, Option<&HitBehaviours>), With<HitBox>>,
    hurt_boxes: Query<&Parent, With<HurtBox>>,
    not_dead: Query<(), Without<Dead>>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity_l, entity_r, flags)
                if flags.contains(CollisionEventFlags::SENSOR) =>
            {
                for (maybe_hit, maybe_hurt) in [(entity_l, entity_r), (entity_r, entity_l)] {
                    if let Ok((hit_parent, hit_behaviours)) = hit_boxes.get(*maybe_hit) {
                        if let Ok(hurt_parent) = hurt_boxes.get(*maybe_hurt) {
                            if let (Ok(()), Ok(())) = (
                                not_dead.get(hurt_parent.get()),
                                not_dead.get(hit_parent.get()),
                            ) {
                                println!("{:?} hit {:?}", hit_parent.get(), hurt_parent.get());

                                hit_events.send(HitEvent {
                                    source: hit_parent.get(),
                                    target: hurt_parent.get(),
                                    hit_behaviours: hit_behaviours.cloned(),
                                });
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn apply_hit_behaviour(
    mut hit_events: EventReader<HitEvent>,
    mut damage_events: EventWriter<DamageEvent>,
    mut kill_events: EventWriter<KillEvent>,
    mut heal_events: EventWriter<HealEvent>,
) {
    let mut new_damage_events = vec![];
    let mut new_kill_events = vec![];
    let mut new_heal_events = vec![];

    for hit_event in hit_events.iter() {
        if let Some(hit_behaviours) = &hit_event.hit_behaviours {
            for hit_behaviour in hit_behaviours.hit_behaviours.iter() {
                match hit_behaviour {
                    HitBehaviour::Damage {
                        override_source,
                        affect_self,
                        amount,
                        kind,
                    } => {
                        let target = if *affect_self {
                            hit_event.source
                        } else {
                            hit_event.target
                        };
                        let source = if let Some(source) = override_source {
                            *source
                        } else {
                            hit_event.source
                        };
                        new_damage_events.push(DamageEvent {
                            source,
                            target,
                            amount: *amount,
                            kind: *kind,
                        })
                    }
                    HitBehaviour::Kill {
                        affect_self,
                        fade_time,
                    } => {
                        let target = if *affect_self {
                            hit_event.source
                        } else {
                            hit_event.target
                        };
                        new_kill_events.push(KillEvent::with_fade_time(
                            hit_event.source,
                            target,
                            *fade_time,
                        ))
                    }
                    HitBehaviour::Heal {
                        affect_self,
                        amount,
                    } => {
                        let target = if *affect_self {
                            hit_event.source
                        } else {
                            hit_event.target
                        };
                        new_heal_events.push(HealEvent {
                            source: hit_event.source,
                            target,
                            amount: *amount,
                        })
                    }
                }
            }
        }
    }

    damage_events.send_batch(new_damage_events);
    kill_events.send_batch(new_kill_events);
    heal_events.send_batch(new_heal_events);
}
