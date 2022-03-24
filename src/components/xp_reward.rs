pub use bevy::prelude::Component;

#[derive(Component)]
pub struct XpReward {
    reward:f32,
}
impl XpReward {
    pub fn new (reward:f32) -> Self {
        Self {
            reward,
        }
    }
    pub fn get(&self) -> f32 { self.reward }
}