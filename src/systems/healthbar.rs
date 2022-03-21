use bevy::prelude::{Query, With, Transform, Vec3, Sprite};
use crate::components::{Healthbar, Health, Playable};

pub fn update_healthbar(
    mut healthbar_query: Query<(&mut Transform, &Sprite), With<Healthbar>>,
    player_query: Query<&Health, With<Playable>>,
) {
    let (mut healthbar, sprite) = healthbar_query.single_mut();
    let health = player_query.single();
    let health_ratio = health.get_health() / health.get_max_health();
    
    // Set health width
    healthbar.scale = Vec3::new(health_ratio, 1.0, 1.0);

    // Align health position with left of container
    let width = sprite.custom_size.unwrap().x;
    let left_space = ((1.0 - health_ratio) * width) / 2.0;
    healthbar.translation.x = -left_space;
}