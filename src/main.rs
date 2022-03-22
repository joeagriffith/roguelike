use bevy::prelude::*;

mod config;
mod entities;
mod items;
mod components;
mod systems;


use entities::*;
use config::{WIDTH, ASPECT_RATIO, TITLE_FONT, TEXT_FONT, BUFFER};
use items::*;
use components::{move_moveables, update_lifetimes, Scoreboard, Healthbar};
use systems::*;


#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
enum SystemLabels {
    Input,
    Movement,
    Animation,
    Teardown,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Playing,
    GameOver,
}

#[derive(Default)]
pub struct Game {
    kills: usize,                                                                     
}
impl Game {
    pub fn increment_kills(&mut self) {
        self.kills += 1;
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Roguelike".to_string(),
            width: WIDTH,
            height: WIDTH / ASPECT_RATIO,
            ..Default::default()
        })
        .init_resource::<Game>()
        .insert_resource(ClearColor(Color::rgb(0.25, 0.03, 0.175)))
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Playing)
        .add_startup_system(setup_camera)
        .add_system_set(SystemSet::on_enter(GameState::Playing)
            .with_system(reset_game)
            // .with_system(spawn_w_meteor)
            .with_system(spawn_player.chain(spawn_w_meteor))
            .with_system(load_level)
            .with_system(init_hud)
            .with_system(spawn_kobold_spawner)
        )
        .add_system_set(SystemSet::on_update(GameState::Playing)
            .with_system(update_guns)
            .with_system(update_lifetimes)
            .with_system(keyboard_input.label(SystemLabels::Input))
            .with_system(target_player.label(SystemLabels::Input))
            .with_system(move_moveables.label(SystemLabels::Movement).after(SystemLabels::Input))
            .with_system(animate_spritesheet.label(SystemLabels::Animation).after(SystemLabels::Movement))
            .with_system(camera_follow_player.after(SystemLabels::Movement))
            .with_system(update_scoreboard)
            .with_system(player_hostile_check)
            .with_system(update_healthbar)
            .with_system(friendly_collision_check)
            .with_system(update_spawners)
        )
        .add_system_set(SystemSet::on_exit(GameState::Playing)
            .with_system(teardown) 
        )
        .add_system_set(SystemSet::on_enter(GameState::GameOver)
            // .with_system(teardown.label(SystemLabels::Teardown))
            .with_system(load_level)
        )
        .add_system_set(SystemSet::on_update(GameState::GameOver)
            .with_system(restart_check)
        )
        .add_system_set(SystemSet::on_exit(GameState::GameOver)
            .with_system(teardown)
        )
        .run();
}

fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn init_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<Camera>>,
) {
    let camera = query.single();

    // Scoreboard
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            "Kills:",
            TextStyle {
                font: asset_server.load(TEXT_FONT),
                font_size: 30.0,
                color: Color::rgb(0.5, 1.0, 0.5),
            },
            Default::default(),
        ),
        transform: Transform::from_translation(Vec3::new(
            -WIDTH/2.0 + BUFFER,
            (WIDTH/ASPECT_RATIO)/2.0 - BUFFER,
            0.0,
        )),
        ..Default::default()
    }).insert(Parent(camera)).insert(Scoreboard{});

    

    // Healthbar
    let healthbar = commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("bar.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 50.0, -1.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::splat(0.4),
        },
        ..Default::default()
    }).insert(Parent(camera)).id();

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("red.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(190.0, 14.0)),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        ..Default::default()
    }).insert(Parent(healthbar)).insert(Healthbar{});
}

fn reset_game(
    mut game: ResMut<Game>,
) {
    game.kills = 0;
}