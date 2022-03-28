use bevy::prelude::*;
// use bevy::utils::Duration;

#[derive(Component)]
pub struct Projectile {
    pub pierce: usize,
    pub destroy_on_impact: bool,
    
}
impl Projectile {
    pub fn pierce(&mut self) -> bool {
        self.pierce -= 1;
        if self.pierce == 0 {
            true
        } else {
            false
        }
    }
}