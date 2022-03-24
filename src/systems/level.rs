use bevy::prelude::*;
use crate::config::{WIDTH, ASPECT_RATIO, TITLE_FONT};
use crate::{GameState};

pub fn load_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State<GameState>>,
) {
    let bundles: (Vec<SpriteBundle>, Vec<Text2dBundle>);
    match state.current() {
        GameState::Playing => bundles = lvl_1_bundles(asset_server),
        GameState::GameOver => bundles = gameover_bundles(asset_server),
    }
    for bundle in bundles.0 { commands.spawn_bundle(bundle); }
    for bundle in bundles.1 { commands.spawn_bundle(bundle); }
}


fn lvl_1_bundles(asset_server: Res<AssetServer>) -> (Vec<SpriteBundle>, Vec<Text2dBundle>) {
    let mut sprites:Vec<SpriteBundle> = Vec::new();
    let texts: Vec<Text2dBundle> = Vec::new();

    // Background
    sprites.push(SpriteBundle {
        texture: asset_server.load("bg_lvl_1.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -0.01),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(2.0, 2.0, 1.0),
        },
        ..Default::default()
    });

    (sprites, texts)
}

fn gameover_bundles(asset_server: Res<AssetServer>) -> (Vec<SpriteBundle>, Vec<Text2dBundle>) {
    let mut sprites:Vec<SpriteBundle> = Vec::new();
    let mut texts: Vec<Text2dBundle> = Vec::new();

    // Background
    sprites.push(SpriteBundle {
        texture: asset_server.load("bg_gameover.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -0.01),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(WIDTH/1280.0, (WIDTH/ASPECT_RATIO)/720.0, 1.0),
        },
        ..Default::default()
    });

    let center_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };

    // Gameover text
    let mut main_text = Text2dBundle {
        text: Text::with_section(
            "GAME OVER",
            TextStyle {
                font: asset_server.load(TITLE_FONT),
                font_size: 100.0,
                color: Color::WHITE,
            },
            center_alignment,
        ),
        ..Default::default()
    };

    main_text.text.sections.push(TextSection {
        value: "\nLoading...".to_string(),
        style: TextStyle {
            font: asset_server.load(TITLE_FONT),
            font_size: 50.0,
            color: Color::WHITE,
        },
    });

    main_text.text.sections.push(TextSection {
        value: format!("\n\n\n\n\n\n\npress spacebar to try again"),
        style: TextStyle {
            font: asset_server.load(TITLE_FONT),
            font_size: 50.0,
            color: Color::WHITE,
        },
    });

    texts.push(main_text);

    (sprites, texts)
}