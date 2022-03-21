use bevy::prelude::{Query, With, Res, Text};
use crate::Game;
use crate::components::Scoreboard;

pub fn update_scoreboard(
    game: Res<Game>,
    mut query: Query<&mut Text, With<Scoreboard>>,
) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("Kills: {}", game.kills);
}