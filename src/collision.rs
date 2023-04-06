use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionEvent;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollidedEvent>()
            .add_system(test);
    }
}

#[derive(Debug, Component)]
pub enum CollisionGroup {
    Player,
    Enemy,
}

pub struct CollidedEvent {
    pub entity_l: Entity,
    pub entity_r: Entity,
}

fn test(
    mut collision_events: EventReader<CollisionEvent>,
    mut collided_events: EventWriter<CollidedEvent>,
    collision_groups: Query<&CollisionGroup>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity_l, entity_r, _) => {
                match (collision_groups.get(*entity_l), collision_groups.get(*entity_r)) {
                    (Ok(CollisionGroup::Player), Ok(CollisionGroup::Enemy)) | (Ok(CollisionGroup::Enemy), Ok(CollisionGroup::Player))=> {
                        collided_events.send(CollidedEvent { entity_l: *entity_l, entity_r: *entity_r});
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }
}
