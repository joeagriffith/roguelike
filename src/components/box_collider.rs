use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub struct BoxCollider {
    width:f32,
    height:f32,
}
impl BoxCollider {
    pub fn new( width:f32, height:f32 ) -> Self {
        Self {
            width,
            height,
        }
    }
    pub fn get_size(&self) -> Vec2 { Vec2::new(self.width, self.height )}
}