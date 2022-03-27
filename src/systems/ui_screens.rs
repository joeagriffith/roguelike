use bevy::prelude::*;

use crate::components::{UI};
use crate::config::{WIDTH, ASPECT_RATIO};


pub fn spawn_pause_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let node_width = 0.35 * WIDTH/ASPECT_RATIO;
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Px(node_width), Val::Percent(100.0)),
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(WIDTH - node_width),
                top: Val::Px(0.0),
                ..Default::default()
            },
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        color: Color::NONE.into(),
        ..Default::default() 
    }).insert(UI).with_children(|parent| {
        parent.spawn_bundle(item_slot(&asset_server));
        parent.spawn_bundle(item_slot(&asset_server));
        parent.spawn_bundle(item_slot(&asset_server));
        parent.spawn_bundle(item_slot(&asset_server));
    });
}

fn item_slot(asset_server: &Res<AssetServer>) -> ImageBundle {
    ImageBundle { 
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
            flex_shrink: 0.0,
            ..Default::default()
        },
        image: asset_server.load("ui/slot.png").into(),
        ..Default::default()
    }
}

pub fn ui_teardown(
    mut commands: Commands,
    query: Query<Entity, With<UI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}