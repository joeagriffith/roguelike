use bevy::prelude::*;

mod config;
mod entities;
mod items;
mod components;
mod systems;
mod utils;


use entities::*;
use config::{WIDTH, ASPECT_RATIO, TEXT_FONT, BUFFER};
use items::*;
use components::{move_moveables, update_lifetimes, update_health, Scoreboard, HealthBar, XpBar};
use systems::*;
use utils::*;


#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
enum SystemLabels {
    Input,
    Movement,
    Animation,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Roguelike".to_string(),
            width: WIDTH,
            height: WIDTH / ASPECT_RATIO,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.25, 0.03, 0.175)))
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Playing)
        .add_startup_system(setup_camera)
        .add_event::<KillEvent>()
        .add_event::<GameOverEvent>()
        .add_system_set(SystemSet::on_enter(GameState::Playing)
            .with_system(spawn_player.chain(spawn_w_meteor))
            .with_system(load_level)
            .with_system(init_hud)
            .with_system(spawn_kobold_spawner)
        )
        .add_system_set(SystemSet::on_update(GameState::Playing)
            .with_system(update_health)
            .with_system(update_guns)
            .with_system(update_lifetimes)
            .with_system(keyboard_input.label(SystemLabels::Input))
            .with_system(target_player.label(SystemLabels::Input))
            .with_system(move_moveables.label(SystemLabels::Movement).after(SystemLabels::Input))
            .with_system(animate_spritesheet.label(SystemLabels::Animation).after(SystemLabels::Movement))
            .with_system(camera_follow_player.after(SystemLabels::Movement))
            .with_system(player_hostile_collision_check)
            .with_system(update_healthbar)
            .with_system(update_xpbar)
            .with_system(handle_kill_event)
            .with_system(friendly_hostile_collision_check)
            .with_system(update_spawners)
        )
        .add_system_set(SystemSet::on_exit(GameState::Playing)
            .with_system(teardown) 
            .with_system(reset_camera)
        )
        .add_system_set(SystemSet::on_enter(GameState::GameOver)
            // .with_system(teardown.label(SystemLabels::Teardown))
            .with_system(load_level)
        )
        .add_system_set(SystemSet::on_update(GameState::GameOver)
            .with_system(handle_gameover_event)
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

fn reset_camera(
    mut query: Query<&mut Transform, With<Camera>>
) {
    query.single_mut().translation = Vec3::new(0.0, 0.0, 999.9);

}

fn init_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<Entity, With<Camera>>,
) {
    let camera = query.single();

    // Scoreboard
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            "Kills:\nLevel:",
            TextStyle {
                font: asset_server.load(TEXT_FONT),
                font_size: 50.0,
                color: Color::rgb(1.0, 1.0, 1.0),
            },
            Default::default(),
        ),
        transform: Transform::from_translation(Vec3::new(
            -WIDTH/2.0 + BUFFER,
            (WIDTH/ASPECT_RATIO)/2.0 - BUFFER,
            0.0,
        )),
        ..Default::default()
    }).insert(Parent(camera)).insert(Scoreboard);


    

    // Healthbar
    let healthbar = commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("bar.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 50.0, -100.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::splat(0.4),
        },
        ..Default::default()
    }).insert(Parent(camera)).id();



    commands.spawn_bundle(ColorMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad {
            size: Vec2::new(190.0, 14.0),
            flip: false,
        })).into(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        material: materials.add(ColorMaterial::from(Color::RED)),
        ..Default::default()
    }).insert(Parent(healthbar)).insert(HealthBar::new(190.0));


    // XPbar
    let xpbar = commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("bar.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 42.0, -100.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::splat(0.4),
        },
        ..Default::default()
    }).insert(Parent(camera)).id();

    commands.spawn_bundle(ColorMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad {
            size: Vec2::new(190.0, 14.0),
            flip: false,
        })).into(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        material: materials.add(ColorMaterial::from(Color::BLUE)),
        ..Default::default()
    }).insert(Parent(xpbar)).insert(XpBar::new(190.0));

}