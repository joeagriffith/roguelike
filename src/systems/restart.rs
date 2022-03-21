use bevy::prelude::{Res, ResMut, State, Input, KeyCode};
use crate::{GameState};

pub fn restart_check(
    mut state: ResMut<State<GameState>>, 
    keyboard_input: Res<Input<KeyCode>>
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set(GameState::Playing).unwrap();
    }
}