use bevy::ecs::query;
use bevy::prelude::*;
use bevy::utils::Duration;
use bevy::input::{keyboard::KeyCode, Input};

use std::f32::consts::PI;

const WIDTH:f32 = 1440.0;
const ASPECT_RATIO:f32 = 4.0/3.0;

const PLAYER_SPEED:f32 = 5.0;

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
enum PlayerActivity {
    Movement,
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
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>, 
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

// Vec to Dir dictionary
const UP3:[f32;3] =        [0.0, 1.0, 0.0];
const RIGHT3:[f32;3] =     [1.0, 0.0, 0.0];
const DOWN3:[f32;3] =      [0.0, -1.0, 0.0];
const LEFT3:[f32;3] =      [-1.0, 0.0, 0.0];
const UPRIGHT3:[f32;3] =   [1.0, 1.0, 0.0];
const UPLEFT3:[f32;3] =    [-1.0, 1.0, 0.0];
const DOWNRIGHT3:[f32;3] = [1.0, -1.0, 0.0];
const DOWNLEFT3:[f32;3] =  [-1.0, -1.0, 0.0];


#[derive(PartialEq)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

#[derive(Component)]
struct Player {
    is_moving: bool,
    speed: f32,
    direction: Direction,
    dir3d: Vec3,
}

#[derive(Component)]
struct Projectile {
    // velocity: Vec3,
    speed: f32,
    lifetime: Duration,
    lifetime_timer: Duration,
}

#[derive(Component)]
struct Weapon {
    texture: String,
    scale: f32,

    cooldown: Duration,
    cooldown_timer: Duration,
    projectile_speed: f32,
}


fn update_weapons_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut wep_query: Query<(&mut Weapon)>,
        player_query: Query<(&Player, &Transform, &TextureAtlasSprite)>,
    time: Res<Time>,

){
    let (player, player_transform, sprite) = player_query.get_single().unwrap();
    for mut weapon in wep_query.iter_mut() {
        // timer.tick(time.delta());
        // if timer.finished() {
        weapon.cooldown_timer += time.delta();
        if weapon.cooldown_timer > weapon.cooldown {
            weapon.cooldown_timer = Duration::from_secs_f32(0.0);
            let mut flip = 1.0;
            if sprite.flip_x { flip = -1.0; }
            commands
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load(&weapon.texture),
                    transform: Transform {
                        translation: player_transform.translation,
                        scale: Vec3::new(1.0, 1.0, 1.0) * weapon.scale,
                        rotation: Quat::from_rotation_z(-player.dir3d.angle_between(Vec3::Y) * flip),
                    },
                    // transform: Transform::from_scale(Vec3::splat(weapon.scale)),
                    ..Default::default()
                })
                .insert( Projectile { 
                    // velocity: transform.forward() * weapon.projectile_speed,
                    speed: weapon.projectile_speed,
                    lifetime: Duration::from_secs(1),
                    lifetime_timer: Duration::from_secs(0),
                });
        }
    }
}

fn projectile_movement_system( mut query: Query<(&mut Projectile, &mut Transform)>, time: Res<Time>) {
    for (mut projectile, mut transform) in query.iter_mut() {
        projectile.lifetime_timer += time.delta();
        let velocity = transform.up() * projectile.speed;
        transform.translation += velocity;
        if projectile.lifetime_timer > projectile.lifetime {

        }
    }
}

fn spawn_w_meteor(mut commands: Commands) {
    commands
        .spawn()
        .insert(Weapon { 
            texture: "meteor.png".to_string(), 
            scale: 0.2,

            cooldown: Duration::from_secs_f32(1.0), 
            cooldown_timer: Duration::from_secs_f32(0.0),
            projectile_speed: 10.0,
        });
        // .insert(Timer::from_seconds(1.0, true));
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(4.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true))
        .insert(Player { 
            speed: PLAYER_SPEED, 
            is_moving: false, 
            direction: Direction::Right,
            dir3d: Vec3::new(1.0, 0.0, 0.0),
        });
}

fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&Player, &mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {

    for (player, mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {

        if player.dir3d.x < 0.0 {
            sprite.flip_x = true;
        } else if player.dir3d.x > 0.0 {
            sprite.flip_x = false;
        }

        if player.is_moving {
            timer.tick(time.delta());
            if timer.finished() {
                let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
                
            }
        } else {
            sprite.index = 0;
        }
    }
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    let (mut player, mut transform) = query.single_mut();
    let mut velocity = Vec3::new(0.0, 0.0, 0.0);

    let mut dir = Vec3::new(0.0, 0.0, 0.0);

    if keyboard_input.pressed(KeyCode::Up) {
        dir.y += 1.0;
        // velocity.y += player.speed;
        // player.direction = Direction::Up;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        dir.y -= 1.0;
        // velocity.y -= player.speed;
        // player.direction = Direction::Down;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        dir.x -= 1.0;
        // velocity.x -= player.speed;
        // player.direction = Direction::Left;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        dir.x += 1.0;
        // velocity.x += player.speed;
        // player.direction = Direction::Right;
    }


    if dir.length() > 0.0 {
        player.dir3d = dir;
        match dir.to_array() {
            UP3        => player.direction = Direction::Up,
            UPRIGHT3   => player.direction = Direction::UpRight,
            RIGHT3     => player.direction = Direction::Right,
            DOWNRIGHT3 => player.direction = Direction::DownRight,
            DOWN3      => player.direction = Direction::Down,
            DOWNLEFT3  => player.direction = Direction::DownLeft,
            LEFT3      => player.direction = Direction::Left,
            UPLEFT3    => player.direction = Direction::UpLeft,
            _ => (),
        }

        player.is_moving = true;
        transform.translation += dir.normalize() * player.speed;
    } else {
        player.is_moving = false;
    }




    // let translation = &mut transform.translation;
    // *translation += velocity;
}