use bevy::prelude::{Query, With, Transform, Vec3};
use crate::components::{HealthBar, Health, Playable, Experience, XpBar};


pub fn update_healthbar(
    mut healthbar_query: Query<(&mut Transform, &HealthBar)>,
    player_query: Query<&Health, With<Playable>>,
) {
    let (mut transform, healthbar) = healthbar_query.single_mut();
    let health = player_query.single();
    let health_ratio = health.get_health() / health.get_max_health();
    
    // Set health width
    transform.scale = Vec3::new(health_ratio, 1.0, 1.0);

    // Align health position with left of container
    let left_space = ((1.0 - health_ratio) * healthbar.get_width()) / 2.0;
    transform.translation.x = -left_space;
}

pub fn update_xpbar(
    mut xpbar_query: Query<(&mut Transform, &XpBar)>,
    player_query: Query<&Experience, With<Playable>>,
) {
    let (mut transform, xpbar) = xpbar_query.single_mut();
    let xp = player_query.single();
    let xp_ratio = xp.get_xp() / xp.get_max_xp();
    
    // Set health width
    transform.scale = Vec3::new(xp_ratio, 1.0, 1.0);

    // Align health position with left of container
    let left_space = ((1.0 - xp_ratio) * xpbar.get_width()) / 2.0;
    transform.translation.x = -left_space;
}