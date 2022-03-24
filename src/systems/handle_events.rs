use bevy::prelude::*;
use crate::components::{Experience, KillTracker, Scoreboard, Playable};
use crate::{KillEvent, GameOverEvent, LevelUpEvent, GameState};
use crate::config::WIDTH;

pub fn handle_kill_event(
    mut player_query: Query<(&mut Experience, &mut KillTracker), With<Playable>>,
    mut scoreboard_query: Query<&mut Text, With<Scoreboard>>,
    mut event_reader: EventReader<KillEvent>,
    mut event_writer: EventWriter<LevelUpEvent>,
) {
    if !player_query.is_empty() {
        let (mut experience, mut killtracker) = player_query.single_mut();
        let mut text = scoreboard_query.single_mut();
        for event in event_reader.iter() {
            killtracker.increment();
            if experience.gain(event.xp_reward) {
                event_writer.send(LevelUpEvent{});
            }
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

pub fn handle_levelup_event(
    mut event_reader: EventReader<LevelUpEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<GameState>>,
) {
    for event in event_reader.iter() {
        if *state.current() == GameState::Playing {
            state.set(GameState::Paused);
        }


        // commands.spawn_bundle(NodeBundle {
        //     style: Style {
        //         size: Size::new(Val::Percent(50.0), Val::Percent(80.0)),
        //         position_type: PositionType::Absolute,
        //         position: Rect { 
        //             left: Val::Percent(25.0), 
        //             top: Val::Percent(10.0), 
        //             ..Default::default()
        //         },
        //         justify_content: JustifyContent::Center,
        //         ..Default::default()
        //     },
        //     color: Color::RED.into(),
        //     ..Default::default()
        // }).with_children(|parent| {
        //     parent.spawn_bundle(ImageBundle {
        //         style: Style {
        //             size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
        //             ..Default::default()
        //         },
        //         image: asset_server.load("ui/border.png").into(),
        //         ..Default::default()
        //     });
        // });


        commands.spawn_bundle(ImageBundle {
            style: Style {
                size: Size::new(Val::Percent(50.0), Val::Percent(80.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(25.0),
                    top: Val::Percent(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            image: asset_server.load("ui/border.png").into(),
            ..Default::default()
        }).with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Percent(90.8), Val::Percent(81.0)),
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Percent((100.0-90.8)/2.0),
                        top: Val::Percent(13.5),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                image: asset_server.load("ui/bg.png").into(),
                ..Default::default()
            }).with_children(|parent| {
                parent.spawn_bundle
            });
        });
    }
}