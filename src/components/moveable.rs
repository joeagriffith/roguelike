use bevy::prelude::{Component, Query, Transform, Vec3};

#[derive(Component)]
pub struct Moveable {
    is_moving: bool,
    speed: f32,
    direction: Vec3,
}
impl Moveable {
    pub fn new(speed:f32) -> Self {
        Self {
            is_moving: true,
            speed,
            direction: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    pub fn is_moving(&self) -> bool { self.is_moving }
    // pub fn get_speed(&self) -> f32 { self.speed }
    pub fn get_direction(&self) -> Vec3 { self.direction}

    pub fn set_moving(&mut self, tf:bool) { self.is_moving = tf; }
    // pub fn set_speed(&mut self, speed:f32) { self.speed = speed; }
    pub fn set_direction(&mut self, dir:Vec3) { self.direction = dir;}
}

pub fn move_moveables (
    mut query: Query<(&Moveable, &mut Transform)>
) {
    for (moveable, mut transform) in query.iter_mut() {
        if moveable.is_moving() {
            transform.translation += moveable.direction * moveable.speed;
        }
    }
}