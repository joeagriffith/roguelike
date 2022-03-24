
use bevy::prelude::Component;

#[derive(Component)]
pub struct KillTracker {
    kills: usize,
}
impl KillTracker {
    pub fn new() -> Self {
        Self {
            kills: 0,
        }
    }
    pub fn get_kills(&self) -> usize { self.kills }
    pub fn increment(&mut self) { self.kills += 1; }
}