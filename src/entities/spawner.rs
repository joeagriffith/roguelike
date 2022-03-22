use bevy::prelude::*;
use rand::prelude::*;
use std::f32::consts::PI;
use crate::components::{Playable, Hostile, Moveable, Health, BoxCollider, Damage};
use crate::config::WIDTH;

#[derive(Component)]
pub struct Spawner {
    texture_filename: String,
    rows: usize,
    cols: usize,
    scale: f32,
    size: Vec2,
    sprite_speed: f32,
    move_speed: f32,
    max_health: f32,
    damage: f32,
    timer: Timer,
}

pub fn spawn_kobold_spawner(
    mut commands: Commands,
) {
    commands
        .spawn()
        .insert(kobold_spawner());
}

fn kobold_spawner() -> Spawner {
    Spawner {
        texture_filename: "kobold-idle.png".to_string(),
        rows: 1,
        cols: 15,
        scale: 4.0,
        size: Vec2::new(24.0, 24.0),
        sprite_speed: 0.1,
        move_speed: 3.0,
        max_health: 100.0,
        damage: 5.0,
        timer: Timer::from_seconds(1.0, true),
    }
}


fn random_spawn_location( player_translation: Vec3 ) -> Vec3 {
    let rand_angle = rand::thread_rng().gen_range(0.0..2.0*PI);
    player_translation + (0.75 * WIDTH * Quat::from_rotation_z(rand_angle).mul_vec3(Vec3::X))
}

pub fn update_spawners(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: Res<Time>,
    player_query: Query<&Transform, With<Playable>>,
    mut spawner_query: Query<&mut Spawner>
) {
    let player_translation = player_query.single().translation;
    for mut spawner in spawner_query.iter_mut() {
        spawner.timer.tick(time.delta());
        if spawner.timer.finished() {
            let texture_handle = asset_server.load(&spawner.texture_filename);
            let texture_atlas = TextureAtlas::from_grid(texture_handle, spawner.size, spawner.cols, spawner.rows);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    transform: Transform {
                        rotation: Quat::IDENTITY,
                        translation: random_spawn_location(player_translation),
                        scale: Vec3::splat(spawner.scale),
                    },
                    ..Default::default()
                })
                .insert(Timer::from_seconds(spawner.sprite_speed, true))
                .insert(Hostile{})
                .insert(Moveable::from_speed(spawner.move_speed))
                .insert(Health::new(spawner.max_health))
                .insert(BoxCollider::new(spawner.size * spawner.scale))
                .insert(Damage::new(spawner.damage));
        }
    }
}