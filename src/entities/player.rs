use bevy::prelude::*;
use bevy::input::{keyboard::KeyCode, Input};

use crate::config::{PLAYER_SPEED};
use crate::components::{Moveable};


#[derive(Component)]
pub struct Playable {
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
        .insert(Playable{})
        .insert(Moveable::new(PLAYER_SPEED));
}

pub fn keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Moveable, With<Playable>>,
) {
    let mut moveable = query.single_mut();

    let mut dir = Vec3::new(0.0, 0.0, 0.0);

    if keyboard_input.pressed(KeyCode::Up)      { dir.y += 1.0; }
    if keyboard_input.pressed(KeyCode::Down)    { dir.y -= 1.0; }
    if keyboard_input.pressed(KeyCode::Left)    { dir.x -= 1.0; }
    if keyboard_input.pressed(KeyCode::Right)   { dir.x += 1.0; }

    dir = dir.normalize();

    if dir.length() > 0.0 {
        moveable.set_direction(dir);
        moveable.set_moving(true);
    } else {
        moveable.set_moving(false);
    }
}



