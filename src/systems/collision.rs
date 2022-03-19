use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use crate::components::{Friendly, Damage, Hostile, Health, BoxCollider};

// Checks for collisions between player shot projectiles and hostiles
pub fn friendly_collision_check(
        mut commands: Commands,
        mut friendly_query: Query<(Entity, &Transform, &BoxCollider, &Damage), With<Friendly>>,
        mut hostile_query: Query<(Entity, &Transform, &BoxCollider, &mut Health), With<Hostile>>,
) {

    for (friendly_entity, friendly_transform, friendly_collider, friendly_damage) in friendly_query.iter_mut() {
        let friendly_size = friendly_collider.get_size();
        for (hostile_entity, hostile_transform, hostile_collider, mut hostile_health) in hostile_query.iter_mut() {

            let collision = collide(
                friendly_transform.translation,
                friendly_size,
                hostile_transform.translation,
                hostile_collider.get_size(),
            );

            if collision.is_some() {
                let health = hostile_health.get_health() - friendly_damage.get_damage();
                if health <= 0.0 {
                    commands.entity(hostile_entity).despawn();
                }
                else {
                    hostile_health.set_health(health);
                }
                commands.entity(friendly_entity).despawn();
                break;
            }
        }
    }
}