use bevy::prelude::{State, Vec2, Commands, Res, AssetServer, SpriteBundle, Transform, Vec3, Quat};
use crate::config::{WIDTH, ASPECT_RATIO};
use crate::GameState;

struct Map {
    texture: String,
    size: Vec2,
    scale: Vec3,
}

pub fn load_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State<GameState>>,
) {

    let level:Map;
    match state.current() {
        GameState::Playing => level = lvl_1(),
        GameState::GameOver => level = gameover(),
    }
    commands.spawn_bundle(bg_spritebundle(level, asset_server));
}

fn bg_spritebundle(map:Map, asset_server: Res<AssetServer>) -> SpriteBundle {
    SpriteBundle {
        texture: asset_server.load(&map.texture),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -0.01),
            rotation: Quat::IDENTITY,
            scale: map.scale,
        },
        ..Default::default()
    }
}

fn lvl_1() -> Map {
    Map {
        texture: "bg_level1.png".to_string(),
        size: Vec2::new(1440.0, 960.0),
        scale: Vec3::new(2.0, 2.0, 1.0),
    }
}

fn gameover() -> Map {
    Map {
        texture: "bg_gameover.png".to_string(),
        size: Vec2::new(1280.0, 720.0),
        scale: Vec3::new(WIDTH/1280.0, (WIDTH/ASPECT_RATIO)/720.0, 1.0),
    }
}
