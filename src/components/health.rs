use bevy::prelude::{Component};

#[derive(Component)]
pub struct Health {
    max_health: f32,
    health: f32,
}
impl Health {
    pub fn new( max_health:f32 ) -> Self {
        Self {
            max_health,
            health: max_health,
        }
    }
    pub fn get_health(&self) -> f32 { self.health }
    pub fn set_health(&mut self, health:f32) { self.health = health}
    pub fn get_max_health(&self) -> f32 { self.max_health }
    // pub fn set_max_health(&mut self, max_health:f32) { self.max_health = max_health;} 

    pub fn damage(&mut self, amount:f32) {
        self.health -= amount;
    }

    pub fn heal(&mut self, amount: f32) {
        self.health += amount;
        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }
}