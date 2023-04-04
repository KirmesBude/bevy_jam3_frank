use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovedEvent>()
            .configure_sets(
                (
                    MovementSet::PreUpdate,
                    MovementSet::Update,
                    MovementSet::PostUpdate,
                )
                    .chain(),)
            .add_system(reset_velocity_vector.in_set(MovementSet::PreUpdate))
            .add_system(apply_velocity_vector.in_set(MovementSet::PostUpdate));
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum MovementSet {
    PreUpdate,
    Update,
    PostUpdate,
}

#[derive(Debug)]
pub struct MovedEvent {
    pub entity: Entity,
    pub distance: f32,
} 

#[derive(Debug, Default, Component)]
pub struct VelocityVector(pub Vec2);

#[derive(Debug, Default, Component)]
pub struct ContinousVelocityVector(pub Vec2);

fn reset_velocity_vector(mut velocity_vectors: Query<&mut VelocityVector>) {
    for mut velocity_vector in velocity_vectors.iter_mut() {
        velocity_vector.0 = Vec2::ZERO;
    }
}

fn apply_velocity_vector(time: Res<Time>, mut query: Query<(Entity, &mut Transform, AnyOf<(&VelocityVector, &ContinousVelocityVector)>)>, mut moved_event: EventWriter<MovedEvent>) {
    for (entity, mut transform, any_velocity) in query.iter_mut() {
        let velocity_vector = match any_velocity {
            (Some(vel), _) => vel.0,
            (_, Some(vel)) => vel.0,
            _ => unreachable!(),
        };
        let velocity_vector = velocity_vector.extend(0.0) * time.delta_seconds();
        transform.translation += velocity_vector;

        // Maybe only send this if a specific component/option exists on the entity?
        let distance = velocity_vector.length();
        moved_event.send(MovedEvent { entity, distance});
    }
}