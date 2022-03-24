use bevy::prelude::Component;

#[derive(Component)]
pub struct HealthBar {
    width:f32,
}
impl HealthBar {
    pub fn new(width:f32) -> Self {
        Self {
            width,
        }
    }
    pub fn get_width(&self) -> f32 { self.width }
}

#[derive(Component)]
pub struct XpBar {
    width:f32,
}
impl XpBar {
    pub fn new(width:f32) -> Self {
        Self {
            width,
        }
    }
    pub fn get_width(&self) -> f32 { self.width }
}