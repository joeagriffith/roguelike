use bevy::prelude::{Component, EventWriter};
use crate::{LevelUpEvent};

#[derive(Component)]
pub struct Experience {
    xp: f32,
    max_xp: f32,
    level: usize,
}
impl Experience {
    pub fn new(max_xp:f32) -> Self {
        Self {
            xp: 0.0,
            max_xp,
            level: 1,
        }
    }
    // Returns true on levelup
    pub fn gain(&mut self, amount:f32) -> bool {
        self.xp += amount;
        if self.xp >= self.max_xp {
            println!("Level Up!");
            self.level += 1;
            self.xp -= self.max_xp;
            self.max_xp *= 1.25;
            true
        } else {
            false
        }
    }
    pub fn get_xp(&self) -> f32 { self.xp }
    pub fn get_level(&self) -> usize { self.level }
    pub fn get_max_xp(&self) -> f32 { self.max_xp }
}