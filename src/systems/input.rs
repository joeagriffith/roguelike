use bevy::prelude::{Res, Input, KeyCode, Query, With, Vec3, ResMut, State};
use crate::components::{Playable, Moveable};
use crate::GameState;

pub fn keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Moveable, With<Playable>>,
    mut state: ResMut<State<GameState>>,
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


    if keyboard_input.just_pressed(KeyCode::Escape) {
        state.set(GameState::Paused);
    }
}

pub fn restart_check(
    mut state: ResMut<State<GameState>>, 
    keyboard_input: Res<Input<KeyCode>>
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set(GameState::GameInit).unwrap();
    }
}

pub fn resume_check(
    mut state: ResMut<State<GameState>>, 
    keyboard_input: Res<Input<KeyCode>>
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set(GameState::Playing).unwrap();
    }
}