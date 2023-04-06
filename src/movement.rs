use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovedEvent>()
            .add_system(save_position_ll.in_base_set(CoreSet::PostUpdate))
            .add_system(send_moved_event.in_base_set(CoreSet::Update));
    }
}

pub struct MovedEvent {
    pub entity: Entity,
    pub distance: f32,
}

#[derive(Debug, Default, Component)]
pub struct PositionLL(pub Vec2);

impl PositionLL {
    pub fn from_transform(transform: &Transform) -> Self {
        Self(
            transform.translation.truncate()
        )
    }
}

fn save_position_ll(mut query: Query<(&Transform, &mut PositionLL)>) {
    for (transform, mut position_ll) in query.iter_mut() {
        position_ll.0 = transform.translation.truncate();
    }
}

pub fn send_moved_event(mut moved_events: EventWriter<MovedEvent>, query: Query<(Entity, &Transform, &PositionLL)>) {
    for (entity, transform, position_ll) in query.iter() {
        let distance = transform.translation.truncate().distance(position_ll.0);

        moved_events.send(MovedEvent {
            entity,
            distance,
        });
    }
}