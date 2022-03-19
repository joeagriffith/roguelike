use bevy::prelude::*;

mod config;
mod player;
mod items;


use player::*;
use config::{WIDTH, ASPECT_RATIO};
use items::*;

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
        .add_system(update_weapons_system)
        .add_system_set(SystemSet::new()
            .with_system(keyboard_input_system.label(PlayerActivity::Movement))
            .with_system(animate_sprite_system.after(PlayerActivity::Movement))
        )
        .add_system(projectile_movement_system)
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



