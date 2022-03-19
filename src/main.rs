use bevy::prelude::*;
use bevy::core::FixedTimestep;

mod config;
mod entities;
mod items;
mod components;
mod systems;


use entities::*;
use config::{WIDTH, ASPECT_RATIO};
use items::*;
use components::{move_moveables, update_lifetimes};
use systems::*
;


#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
enum SystemLabels {
    Input,
    Movement,
    Animation,
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
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_w_meteor)
        .add_system(update_weapons)
        .add_system(update_lifetimes)
        .add_system_set(SystemSet::new()
            .with_system(keyboard_input.label(SystemLabels::Input))
            .with_system(target_player.label(SystemLabels::Input))
            .with_system(move_moveables.label(SystemLabels::Movement).after(SystemLabels::Input))
            .with_system(animate_spritesheet.label(SystemLabels::Animation).after(SystemLabels::Movement))
            .with_system(camera_follow_player.after(SystemLabels::Movement))
        )
        // .add_system(move_moveables_sys)
        // .add_system(projectile_movement)
        .add_system(friendly_collision_check)
        .add_system_set(SystemSet::new()
            .with_run_criteria(FixedTimestep::step(0.25))
            .with_system(spawn_enemy)
        )
        .add_plugins(DefaultPlugins)
        .run();
}


fn setup(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>, 
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}



