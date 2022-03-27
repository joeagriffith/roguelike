use bevy::prelude::*;
use crate::components::{Moveable, BoxCollider, Health, Playable, Experience, KillTracker};
use crate::items::{Weapons, spawn_weapon};
use crate::{NewItemEvent};


struct Player {
    texture: String,
    size: Vec2,
    scale: f32,
    rows: usize,
    cols: usize,
    move_speed: f32,
    max_health: f32,
    default_weapon: Weapons,
}

fn gabe() -> Player {
    Player {
        texture: "gabe-idle-run.png".to_string(),
        size: Vec2::new(24.0, 24.0),
        scale: 4.0,
        rows: 1,
        cols: 7,
        move_speed: 5.0,
        max_health: 100.0,
        default_weapon: Weapons::MeteorBlaster,
    }
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut event_writer: EventWriter<NewItemEvent>,
){

    let player = gabe();

    let texture_handle = asset_server.load(&player.texture);
    let texture_atlas = TextureAtlas::from_grid(texture_handle,player.size, player.cols, player.rows);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let player_entity = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(player.scale)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true))
        .insert(Playable)
        .insert(Moveable::from_speed(player.move_speed))
        .insert(BoxCollider::new(player.size * player.scale))
        .insert(Health::new(player.max_health))
        .insert(Experience::new(2.0))
        .insert(KillTracker::new())
        .id();
    spawn_weapon(&mut commands, player.default_weapon, player_entity, &mut event_writer);
}



