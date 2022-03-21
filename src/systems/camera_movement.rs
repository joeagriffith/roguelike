use bevy::prelude::{QuerySet, QueryState, With, Transform, Camera};
use crate::components::Playable;

pub fn camera_follow_player(
    mut query: QuerySet<(
        QueryState<&Transform, With<Playable>>,
        QueryState<&mut Transform, With<Camera>>,
    )>
    // mut camera_query: Query<&mut Transform, With<Camera>, Without<Playable>>,
    // player_query: Query<&Transform, With<Playable>>,
) {
    let translation = query.q0().single().translation.clone();
    // Has to be handled wierdly due to quirks of QueryState (there is only one camera)
    for mut camera_transform in query.q1().iter_mut() {
        camera_transform.translation.x = translation.x;
        camera_transform.translation.y = translation.y;
    }
}