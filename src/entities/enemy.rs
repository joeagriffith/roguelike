use bevy::prelude::*;
use rand::prelude::*;
use std::f32::consts::PI;
use crate::components::Moveable;
use crate::config::WIDTH;
use crate::entities::player::Playable;


#[derive(Component)]
pub struct Hostile {
}

pub fn spawn_enemy(
    commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    query: Query<&Transform, With<Playable>>
) {
    spawn_hostile_from_spritesheet(
        commands,
        asset_server,
        texture_atlases,
        "kobold-idle.png",
        (1, 15),
        4.0,
        0.1,
        3.0,
        query.single().translation,
    )
}

fn spawn_hostile_from_spritesheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    texture_filename: &str,
    (rows, cols): (usize, usize),
    scale: f32,
    sprite_speed: f32,
    move_speed: f32,
    player_translation: Vec3,
) {
    let texture_handle = asset_server.load(texture_filename);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), cols, rows);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                rotation: Quat::IDENTITY,
                translation: random_spawn_location(player_translation),
                scale: Vec3::splat(scale),
            },
            // transform: Transform::from_scale(Vec3::splat(scale)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(sprite_speed, true))
        .insert(Hostile{})
        .insert(Moveable::new(move_speed));
}

fn random_spawn_location( player_translation: Vec3 ) -> Vec3 {
    let rand_angle = rand::thread_rng().gen_range(0.0..2.0*PI);
    player_translation + (0.75 * WIDTH * Quat::from_rotation_z(rand_angle).mul_vec3(Vec3::X))
}