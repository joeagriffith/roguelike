use bevy::prelude::*;
use bevy::utils::Duration;

#[derive(Component)]
pub struct Lifetime {
    lifetime: Duration,
    time_alive: Duration,
}
impl Lifetime {
    pub fn new(lifetime:f32) -> Self {
        Self {
            lifetime: Duration::from_secs_f32(lifetime),
            time_alive: Duration::ZERO,
        }
    }
    fn tick(&mut self, time:Duration) {
        self.time_alive += time;
    }
    fn is_invalid(&self) -> bool {
        self.time_alive > self.lifetime
    }
}

pub fn update_lifetimes (
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.tick(time.delta());
        if lifetime.is_invalid() {
            commands.entity(entity).despawn();
        }
    } 
}

