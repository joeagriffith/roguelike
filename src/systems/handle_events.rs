use bevy::prelude::*;
use crate::components::{Primary, Experience, KillTracker, Scoreboard, Playable, UI};
use crate::{GameState, Inventory};
use crate::utils::*;
use crate::systems::update_inventory;
use crate::items::{Weapons, spawn_weapon};
use crate::NewItemEvent;

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
    player_query: Query<Entity, With<Playable>>,
    mut new_item_event_writer: EventWriter<NewItemEvent>,
) {
    for event in event_reader.iter() {
        if *state.current() == GameState::Playing {
            state.set(GameState::Paused);
        }
        spawn_weapon(&mut commands, Weapons::SolarFlare, player_query.single(), &mut new_item_event_writer);


        commands.spawn_bundle(ImageBundle {
            style: Style {
                size: Size::new(Val::Percent(40.0), Val::Percent(80.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(30.0),
                    top: Val::Percent(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            image: asset_server.load("ui/border.png").into(),
            ..Default::default()
        }).insert(UI).with_children(|parent| {
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
            });
        });
    }
}

pub fn handle_new_item_event(
    mut event_reader: EventReader<NewItemEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut inventory: ResMut<Inventory>,
    camera_query: Query<(Entity, &Primary), With<Camera>>,
) {
    for event in event_reader.iter() {
        println!("Handling new item event");
        let (camera, _flag) = camera_query.single();
        update_inventory(&mut commands, &asset_server, &mut inventory, event, camera);
    }
}