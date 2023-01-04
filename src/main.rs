use bevy::prelude::*;
use bevy::render::primitives::Frustum;

mod config;
mod entities;
mod items;
mod components;
mod systems;
mod utils;


use entities::*;
use config::{WIDTH, ASPECT_RATIO, TEXT_FONT, BUFFER};
use items::*;
use components::*;
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
    GameInit,
    Playing,
    Paused,
    GameOver,
}

pub enum ItemType {
    Weapon,
    Trinket,
}

pub struct Inventory {
    weapon_count: usize,
    trinket_count: usize,
}
impl Inventory {
    fn new() -> Self {
        Self {
            weapon_count: 0,
            trinket_count: 0,
        }
    }
}

fn main() {
    let app = App::new()
        .insert_resource(WindowDescriptor {
            title: "Roguelike".to_string(),
            width: WIDTH,
            height: WIDTH / ASPECT_RATIO,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.25, 0.03, 0.175)))
        .insert_resource(Inventory::new())
        .add_plugins(DefaultPlugins)
        .add_state(GameState::GameInit)
        .add_startup_system(setup_camera)
        .add_event::<KillEvent>()
        .add_event::<GameOverEvent>()
        .add_event::<LevelUpEvent>()
        .add_event::<NewItemEvent>()
        .add_system_set(
            {
                let mut set = SystemSet::on_enter(GameState::GameInit)
            .with_system(new_game)
            .with_system(spawn_player)
            .with_system(load_level)
            .with_system(spawn_scoreboard);
            if true {
                set = set.with_system(spawn_kobold_spawner);
            }
            set
            // .with_system(spawn_kobold_spawner)
            }
        )
        .add_system_set(SystemSet::on_update(GameState::GameInit)
            .with_system(start)
            .with_system(spawn_hud)
        )
        .add_system_set(SystemSet::on_enter(GameState::Playing)
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
            .with_system(handle_levelup_event)
            .with_system(handle_new_item_event)
        )
        .add_system_set(SystemSet::on_enter(GameState::Paused)
            .with_system(spawn_ui_camera)
            .with_system(spawn_pause_ui)
        )
        .add_system_set(SystemSet::on_update(GameState::Paused)
            .with_system(resume_check)
            .with_system(handle_new_item_event)
        )
        .add_system_set(SystemSet::on_exit(GameState::Playing)
            // .with_system(teardown_hud)
        )
        .add_system_set(SystemSet::on_exit(GameState::Paused)
            // .with_system(despawn_ui_camera)
            .with_system(ui_teardown)
        )
        .add_system_set(SystemSet::on_enter(GameState::GameOver)
            // .with_system(teardown.label(SystemLabels::Teardown))
            .with_system(teardown) 
            .with_system(reset_camera)
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

fn spawn_ui_camera(
    mut commands: Commands,
) {
    commands.spawn_bundle(UiCameraBundle{
        ..Default::default()
    }).insert(UI);
}

fn despawn_ui_camera(
    mut commands: Commands,
    mut query: Query<(Entity, &Camera), Without<Frustum>>,
) {
    let (ui_cam, _cam) = query.single_mut();
    commands.entity(ui_cam).despawn();
}

fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(Primary);
}

fn reset_camera(
    mut query: Query<&mut Transform, With<Camera>>
) {
    query.single_mut().translation = Vec3::new(0.0, 0.0, 999.9);
}

fn start(
    mut state: ResMut<State<GameState>>,
) {
    state.set(GameState::Playing);
}

fn new_game(
    mut inventory: ResMut<Inventory>,
) {
    inventory.weapon_count = 0;
    inventory.trinket_count = 0;
}

fn spawn_scoreboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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

}