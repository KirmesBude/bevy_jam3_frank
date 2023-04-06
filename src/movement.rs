use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovedEvent>()
            .add_system(send_moved_event_and_update_position_ll.in_base_set(CoreSet::PostUpdate));
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
        Self(transform.translation.truncate())
    }
}

fn send_moved_event_and_update_position_ll(
    mut moved_events: EventWriter<MovedEvent>,
    mut query: Query<(Entity, &Transform, &mut PositionLL), Changed<Transform>>,
) {
    for (entity, transform, mut position_ll) in query.iter_mut() {
        let distance = transform.translation.truncate().distance(position_ll.0);

        if distance > 0.0 {
            moved_events.send(MovedEvent { entity, distance });
        }

        position_ll.0 = transform.translation.truncate();
    }
}
