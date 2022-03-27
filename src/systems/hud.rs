use bevy::prelude::*;
use crate::components::{HealthBar, XpBar, HUD, Inv};
use crate::config::{WIDTH, ASPECT_RATIO};
use crate::{Inventory, ItemType, NewItemEvent};

pub fn spawn_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<Entity, With<Camera>>,
) {
    let camera = query.single();


    // Healthbar
    let healthbar = commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("bar.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 50.0, -100.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::splat(0.4),
        },
        ..Default::default()
    }).insert(Parent(camera)).insert(HUD).id();
    commands.spawn_bundle(ColorMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad {
            size: Vec2::new(190.0, 14.0),
            flip: false,
        })).into(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        material: materials.add(ColorMaterial::from(Color::RED)),
        ..Default::default()
    }).insert(Parent(healthbar)).insert(HealthBar::new(190.0)).insert(HUD);


    // XPbar
    let xpbar = commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("bar.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 42.0, -100.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::splat(0.4),
        },
        ..Default::default()
    }).insert(Parent(camera)).insert(HUD).id();
    commands.spawn_bundle(ColorMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad {
            size: Vec2::new(190.0, 14.0),
            flip: false,
        })).into(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        material: materials.add(ColorMaterial::from(Color::BLUE)),
        ..Default::default()
    }).insert(Parent(xpbar)).insert(XpBar::new(190.0)).insert(HUD);


    //Inventory
    let slot_length = WIDTH * 0.05;
    let slot_size = Vec2::new(slot_length, slot_length);
    let HEIGHT = WIDTH/ASPECT_RATIO;
    let inv_translation = Vec3::new((WIDTH/2.0)-(3.5*slot_length), (-HEIGHT/2.0)+(0.5*slot_length), 1.0);
    // let inv_translation = Vec3::new((WIDTH/2.0), (-HEIGHT/2.0), 1.0);
    for i in 0..8 {
        let x_pos = inv_translation.x + ((i%4) as f32 * slot_length);
        let mut y_pos = inv_translation.y;
        if i > 3 {y_pos += slot_length;}
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(slot_size),
                ..Default::default()
            },
            texture: asset_server.load("ui/slot.png"),
            transform: Transform::from_translation(Vec3::new(x_pos, y_pos, -800.0)),
            ..Default::default()
        }).insert(Parent(camera)).insert(HUD);
    }
}

pub fn teardown_hud(
    mut commands: Commands,
    query: Query<Entity, With<HUD>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_inventory(
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut inventory: &mut ResMut<Inventory>,
    event: &NewItemEvent,
    camera: Entity,
) {
    println!("new item!");
    let slot_length = WIDTH * 0.05;
    let slot_size = Vec2::new(slot_length, slot_length);
    let HEIGHT = WIDTH/ASPECT_RATIO;
    let inv_translation = Vec3::new((WIDTH/2.0)-(3.5*slot_length), (-HEIGHT/2.0)+(0.5*slot_length), 1.0);
    let x_pos:f32;
    let y_pos:f32;
    
    match event.item_type {
        ItemType::Weapon => {
            println!("weapon_count: {}", inventory.weapon_count);
            x_pos = inv_translation.x + (inventory.weapon_count as f32 * slot_length);
            y_pos = inv_translation.y + slot_length;
            inventory.weapon_count += 1;
        },
        ItemType::Trinket => {
            x_pos = inv_translation.x + (inventory.weapon_count as f32 * slot_length);
            y_pos = inv_translation.y;
            inventory.weapon_count += 1;
        },
    };

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite{
            custom_size: Some(slot_size * 0.8),
            ..Default::default()
        },
        texture: asset_server.load(&event.texture),
        transform: Transform::from_translation(Vec3::new(x_pos, y_pos, -80.0)),
        ..Default::default()
    }).insert(Parent(camera)).insert(HUD);
}