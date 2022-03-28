use bevy::prelude::*;
use bevy::utils::Duration;
use rand::prelude::*;

use crate::components::{Projectile, Moveable, Hostile, Friendly, Damage, BoxCollider, Lifetime, Playable};
use crate::{NewItemEvent, ItemType};

pub enum Weapons {
    MeteorBlaster,
    HomingMG,
    LightningRod,
}

#[derive(Component)]
pub struct Gun {
    weapon: Weapons,
    texture: String,
    proj_size: Vec2,
    proj_scale: f32,
    proj_texture: String,
    proj_speed: f32,
    proj_lifetime: Duration,
    timer: Timer,
    trail: Option<String>,
    damage: f32,
    // shoot_fn: dyn Fn(Commands, Res<AssetServer>, Query<&mut Gun>, Query<(&Transform, &TextureAtlasSprite, &Moveable), With<Playable>>, Res<Time>),
}

pub fn update_guns(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut gun_query: Query<&mut Gun>,
    player_query: Query<(&Transform, &TextureAtlasSprite, &Moveable), With<Playable>>,
    hostile_query: Query<&Transform, With<Hostile>>,
    time: Res<Time>,
){
    let (player_transform, sprite, moveable) = player_query.get_single().unwrap();

    for mut gun in gun_query.iter_mut() {
        match gun.weapon {
            Weapons::MeteorBlaster => {
                gun.timer.tick(time.delta());
                if gun.timer.finished() {

                    let mut flip = 1.0;
                    if sprite.flip_x { flip = -1.0; }
                    let rotation = Quat::from_rotation_z(-moveable.get_direction().angle_between(Vec3::Y) * flip);
                    
                    commands
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(gun.proj_size * gun.proj_scale),
                                ..Default::default()
                            },
                            texture: asset_server.load(&gun.proj_texture),
                            transform: Transform {
                                translation: player_transform.translation,
                                rotation,
                                scale: Vec3::ONE,
                            },
                            ..Default::default()
                        })
                        .insert(Projectile { pierce:2, destroy_on_impact:true, })
                        .insert(Friendly)
                        .insert(Damage::new(100.0))
                        .insert(BoxCollider::new(gun.proj_size * gun.proj_scale))
                        .insert(Lifetime::new(gun.proj_lifetime))
                        .insert(Moveable::new(true, gun.proj_speed, rotation.mul_vec3(Vec3::Y)));
                }
            },

            Weapons::HomingMG => {
                gun.timer.tick(time.delta());
                if gun.timer.finished() {
                    let target_pos = rand_hostile_pos(&hostile_query);
                    
                }
            }

            Weapons::LightningRod => {
                gun.timer.tick(time.delta());
                if gun.timer.finished() {
                    if let Some(target_pos) = rand_hostile_pos(&hostile_query){
                        commands
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(gun.proj_size * gun.proj_scale),
                                ..Default::default()
                            },
                            texture: asset_server.load(&gun.proj_texture),
                            transform: Transform::from_translation(target_pos),
                            ..Default::default()
                        })
                        .insert(Friendly)
                        .insert(Lifetime::new(gun.proj_lifetime))
                        .insert(BoxCollider::new(gun.proj_size * gun.proj_scale))
                        .insert(Damage::new(gun.damage))
                        .insert(Projectile{ pierce:100000, destroy_on_impact:false});
                    }
                }
            }
        }
    }
}

pub fn rand_hostile_pos(
    query: &Query<&Transform, With<Hostile>>,
) -> Option<Vec3> {
    let mut hostile_positions: Vec<Vec3> = Vec::new();
    for transform in query.iter() {
        hostile_positions.push(transform.translation);
    }
    if hostile_positions.len() > 0 {
        let mut rng = rand::thread_rng();
        let rand_index = rng.gen_range(0..hostile_positions.len());
        Some(hostile_positions[rand_index])
    } else {
        None
    }
}

pub fn spawn_weapon(
    mut commands: &mut Commands,
    weapon: Weapons,
    player_entity: Entity,
    mut event_writer: &mut EventWriter<NewItemEvent>,
) {
    let weapon_params = match weapon {
        Weapons::MeteorBlaster => meteor_blaster(),
        Weapons::LightningRod => lightning_rod(),
        Weapons::HomingMG => homing_mg(),
    };
    event_writer.send(NewItemEvent {texture: weapon_params.texture.clone(), item_type: ItemType::Weapon});
    commands
        .spawn()
        .insert(weapon_params)
        .insert(Parent(player_entity));
}


fn meteor_blaster() -> Gun {
    Gun {
        weapon: Weapons::MeteorBlaster,
        texture: "meteor_blaster.png".to_string(),
        trail: None,
        proj_size: Vec2::new(190.0, 420.0),
        proj_scale: 0.1,
        proj_texture: "meteor.png".to_string(),
        proj_speed: 10.0,
        proj_lifetime: Duration::from_secs_f32(1.5),
        timer: Timer::from_seconds(0.5, true),
        damage: 200.0,
    }
}
fn homing_mg() -> Gun {
    Gun {
        weapon: Weapons::HomingMG,
        texture: "homing_mg.png".to_string(),
        trail: None,
        proj_size: Vec2::new(190.0, 420.0),
        proj_scale: 0.1,
        proj_texture: "meteor.png".to_string(),
        proj_speed: 10.0,
        proj_lifetime: Duration::from_secs_f32(1.5),
        timer: Timer::from_seconds(0.5, true),
        damage: 10.0,
    }
}
fn lightning_rod() -> Gun {
    Gun {
        weapon: Weapons::LightningRod,
        texture: "lightning_rod.png".to_string(),
        trail: Some("lightning.png".to_string()),
        proj_size: Vec2::new(150.0, 105.0),
        proj_scale: 1.0,
        proj_texture: "crater.png".to_string(),
        proj_speed: 10.0,
        proj_lifetime: Duration::from_secs_f32(0.75),
        timer: Timer::from_seconds(2.0, true),
        damage: 200.0,
    }
}