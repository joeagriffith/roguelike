use bevy::prelude::{Query, EventReader, Text, With};
use crate::components::{Experience, KillTracker, Scoreboard};
use crate::{KillEvent, GameOverEvent};

// pub fn update_xp(
//     mut query: Query<&mut Experience>,
//     mut event_reader: EventReader<KillEvent>,
// ) {
//     if !query.is_empty() {
//         let mut exp = query.single_mut();
//         for event in event_reader.iter() {
//             exp.gain(event.xp_reward);
//         }
//     }
// }

pub fn handle_kill_event(
    mut player_query: Query<(&mut Experience, &mut KillTracker)>,
    mut scoreboard_query: Query<&mut Text, With<Scoreboard>>,
    mut event_reader: EventReader<KillEvent>,
) {
    if !player_query.is_empty() {
        let (mut experience, mut killtracker) = player_query.single_mut();
        let mut text = scoreboard_query.single_mut();
        for event in event_reader.iter() {
            experience.gain(event.xp_reward);
            killtracker.increment();
        }
        text.sections[0].value = format!("Kills: {}\nLevel: {}", killtracker.get_kills(), experience.get_level());
    }
}

pub fn handle_gameover_event(
    mut event_reader: EventReader<GameOverEvent>,
    mut text_query: Query<&mut Text>
) {
    for event in event_reader.iter() {
        let mut text = text_query.single_mut();
        text.sections[1].value = format!("\n\nKills: {}\nLevel: {}", event.kills, event.level);
    }
}