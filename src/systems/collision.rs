use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use crate::components::{Friendly, Damage, Hostile, Health, BoxCollider, Playable, XpReward};
use crate::utils::{KillEvent};

// Checks for collisions between player shot projectiles and hostiles
pub fn friendly_hostile_collision_check(
        mut commands: Commands,
        mut friendly_query: Query<(Entity, &Transform, &BoxCollider, &Damage), With<Friendly>>,
        mut hostile_query: Query<(Entity, &Transform, &BoxCollider, &mut Health, &XpReward), With<Hostile>>,
        mut event_writer: EventWriter<KillEvent>, 
) {

    for (friendly_entity, friendly_transform, friendly_collider, friendly_damage) in friendly_query.iter_mut() {
        let friendly_size = friendly_collider.get_size();
        for (hostile_entity, hostile_transform, hostile_collider, mut hostile_health, xp_reward) in hostile_query.iter_mut() {

            let collision = collide(
                friendly_transform.translation,
                friendly_size,
                hostile_transform.translation,
                hostile_collider.get_size(),
            );

            if collision.is_some() {
                let health = hostile_health.get_health() - friendly_damage.get();
                if health <= 0.0 {
                    commands.entity(hostile_entity).despawn();

                    event_writer.send(KillEvent { xp_reward: xp_reward.get() })

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

pub fn player_hostile_collision_check(
    mut player_query: Query<(&Transform, &BoxCollider, &mut Health), With<Playable>>,
    hostile_query: Query<(&Transform, &BoxCollider, &Damage), With<Hostile>>,
) {
    let (player_transform, player_collider, mut player_health) = player_query.single_mut();
    let player_size = player_collider.get_size();
    for (hostile_transform, hostile_collider, hostile_damage) in hostile_query.iter() {
        
        let collision = collide(
            player_transform.translation,
            player_size,
            hostile_transform.translation,
            hostile_collider.get_size(),
        );

        if collision.is_some() {
            player_health.damage(hostile_damage.get());
        }
    }
}