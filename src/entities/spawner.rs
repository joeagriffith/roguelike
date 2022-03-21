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
    sprite_speed: f32,
    move_speed: f32,
    max_health: f32,
    damage: f32,
}
impl Spawner {
    pub fn new(
        texture_filename: String,
        rows: usize,
        cols: usize,
        scale: f32,
        sprite_speed: f32,
        move_speed: f32,
        max_health: f32,
        damage: f32,
    ) -> Self {
        Self {
            texture_filename,
            rows,
            cols,
            scale,
            sprite_speed,
            move_speed,
            max_health,
            damage,
        }
    }
}

pub fn spawn_kobold_spawner(
    mut commands: Commands,
    // query: Query<Entity, With<Playable>>,
) {
    // let player = query.single();
    commands
        .spawn()
        .insert(Timer::from_seconds(0.5, true))
        .insert(Spawner::new(
            "kobold-idle.png".to_string(),
            1,
            15,
            4.0,
            0.1,
            3.0,
            100.0,
            5.0,
        ));
        // .insert(Parent(player));
}

fn random_spawn_location( player_translation: Vec3 ) -> Vec3 {
    let rand_angle = rand::thread_rng().gen_range(0.0..2.0*PI);
    player_translation + (0.75 * WIDTH * Quat::from_rotation_z(rand_angle).mul_vec3(Vec3::X))
}
// fn random_spawn_location() -> Vec3 {
//     let rand_angle = rand::thread_rng().gen_range(0.0..2.0*PI);
//     0.75 * WIDTH * Quat::from_rotation_z(rand_angle).mul_vec3(Vec3::X)
// }

pub fn update_spawners(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: Res<Time>,
    player_query: Query<&Transform, With<Playable>>,
    mut spawner_query: Query<(&Spawner, &mut Timer)>
) {
    let player_translation = player_query.single().translation;
    for (spawner, mut timer) in spawner_query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_handle = asset_server.load(&spawner.texture_filename);
            let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), spawner.cols, spawner.rows);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    transform: Transform {
                        rotation: Quat::IDENTITY,
                        translation: random_spawn_location(player_translation),
                        // translation: random_spawn_location(),
                        scale: Vec3::splat(spawner.scale),
                    },
                    ..Default::default()
                })
                .insert(Timer::from_seconds(spawner.sprite_speed, true))
                .insert(Hostile{})
                .insert(Moveable::from_speed(spawner.move_speed))
                .insert(Health::new(spawner.max_health))
                .insert(BoxCollider::new(22.0*spawner.scale, 22.0*spawner.scale))
                .insert(Damage::new(spawner.damage));
        }
    }
}