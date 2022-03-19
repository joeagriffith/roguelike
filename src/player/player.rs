use bevy::prelude::*;
use bevy::input::{keyboard::KeyCode, Input};

use crate::config::{PLAYER_SPEED};


#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerActivity {
    Movement,
}

#[derive(Component)]
pub struct Player {
    is_moving: bool,
    speed: f32,
    direction: Vec3,
}
impl Player {
    // pub fn is_moving(&self) -> bool { self.is_moving }

    // pub fn get_speed(&self) -> f32 { self.speed }
    pub fn get_direction(&self) -> Vec3 { self.direction.clone() }

    // pub fn set_speed(&mut self, speed:f32) { self.speed = speed; }
    // pub fn set_direction(&mut self, dir:Vec3) { self.direction = dir; }
}

pub fn spawn_player(
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
            // direction: Direction::Right,
            direction: Vec3::new(1.0, 0.0, 0.0),
        });
}


pub fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&Player, &mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {

    for (player, mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {

        if player.direction.x < 0.0 {
            sprite.flip_x = true;
        } else if player.direction.x > 0.0 {
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

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    let (mut player, mut transform) = query.single_mut();

    let mut dir = Vec3::new(0.0, 0.0, 0.0);

    if keyboard_input.pressed(KeyCode::Up) {
        dir.y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        dir.y -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        dir.x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        dir.x += 1.0;
    }

    dir = dir.normalize();


    if dir.length() > 0.0 {
        player.direction = dir;
        player.is_moving = true;
        transform.translation += dir * player.speed;
    } else {
        player.is_moving = false;
    }

}