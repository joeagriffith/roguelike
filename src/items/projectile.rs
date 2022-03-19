use bevy::prelude::*;
// use bevy::utils::Duration;

#[derive(Component)]
pub struct Projectile {
    // speed: f32,
    // lifetime: Duration,
    // // lifetime_timer: Duration,
}
// impl Projectile {
    // pub fn new (speed:f32, lifetime: f32) -> Self {
    //     Self  {
    //         speed,
            // lifetime: Duration::from_secs_f32(lifetime),
            // lifetime_timer: Duration::from_secs(0),
        // }
    // }
// }

// pub fn projectile_movement( mut query: Query<(&mut Projectile, &mut Transform)>, time: Res<Time>) {

//     for (mut projectile, mut transform) in query.iter_mut() {
//         projectile.lifetime_timer += time.delta();
//         let velocity = transform.up() * projectile.speed;
//         transform.translation += velocity;
//         if projectile.lifetime_timer > projectile.lifetime {
//         }
//     }
// }