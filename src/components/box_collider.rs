use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub struct BoxCollider {
    width:f32,
    height:f32,
}
impl BoxCollider {
    pub fn new( size: Vec2 ) -> Self {
        Self {
            width: size.x,
            height: size.y,
        }
    }
    pub fn get_size(&self) -> Vec2 { Vec2::new(self.width, self.height )}
}