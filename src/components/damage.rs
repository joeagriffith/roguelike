use bevy::prelude::Component;

#[derive(Component)]
pub struct Damage {
    damage:f32,
}
impl Damage {
    pub fn new( damage:f32 ) -> Self {
        Self {
            damage,
        }
    }
    pub fn get_damage(&self) -> f32 { self.damage }
}