use bevy::prelude::{Res, Input, KeyCode, Query, With, Vec3};
use crate::components::{Playable, Moveable};

pub fn keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Moveable, With<Playable>>,
) {
    let mut moveable = query.single_mut();

    //Only set to 0 if not standing on blood
    let mut dir = Vec3::new(0.0, 0.0, 0.0);

    if keyboard_input.pressed(KeyCode::Up)      { dir.y += 1.0; }
    if keyboard_input.pressed(KeyCode::Down)    { dir.y -= 1.0; }
    if keyboard_input.pressed(KeyCode::Left)    { dir.x -= 1.0; }
    if keyboard_input.pressed(KeyCode::Right)   { dir.x += 1.0; }

    dir = dir.normalize();

    if dir.length() > 0.0 {
        moveable.set_direction(dir);
        moveable.set_moving(true);
    } else {
        moveable.set_moving(false);
    }
}