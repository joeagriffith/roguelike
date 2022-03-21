use bevy::prelude::{Query, Transform, With, Vec3};

use crate::components::{Moveable, Hostile, Playable};

pub fn target_player(
    mut query: Query<(&mut Moveable, &Transform), With<Hostile>>,
    player_query: Query<&Transform, With<Playable>>,
) {
    let player_transform = player_query.get_single().unwrap();
    for (mut moveable, transform) in query.iter_mut() {
        let mut dir = player_transform.translation - transform.translation;
        if dir.length() < 3.0 {
            dir = Vec3::ZERO;
        }
        moveable.set_direction(dir.normalize_or_zero());
    }
}