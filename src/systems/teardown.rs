use bevy::prelude::{Commands, Query, Without, Entity, Camera, DespawnRecursiveExt};
// use crate::



pub fn teardown(
    mut commands: Commands,
    query: Query<Entity, Without<Camera>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
