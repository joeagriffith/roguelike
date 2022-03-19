use bevy::prelude::*;
use bevy::utils::Duration;

use crate::entities::Playable;
use crate::components::{Moveable, Friendly, Damage, BoxCollider, Lifetime};
use super::projectile::Projectile;

#[derive(Component)]
pub struct Weapon {
    texture: String,
    scale: f32,

    cooldown: Duration,
    cooldown_timer: Duration,
    projectile_speed: f32,
    projectile_lifetime: f32,
}

pub fn update_weapons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut weapon_query: Query<&mut Weapon>,
    player_query: Query<(&Transform, &TextureAtlasSprite, &Moveable), With<Playable>>,
    time: Res<Time>,
){
    let (player_transform, sprite, moveable) = player_query.get_single().unwrap();
    for mut weapon in weapon_query.iter_mut() {
        weapon.cooldown_timer += time.delta();
        if weapon.cooldown_timer > weapon.cooldown {
            weapon.cooldown_timer = Duration::from_secs_f32(0.0);
            let mut flip = 1.0;
            if sprite.flip_x { flip = -1.0; }
            let rotation = Quat::from_rotation_z(-moveable.get_direction().angle_between(Vec3::Y) * flip);
            
            commands
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load(&weapon.texture),
                    transform: Transform {
                        translation: player_transform.translation,
                        scale: Vec3::new(1.0, 1.0, 1.0) * weapon.scale,
                        rotation,
                    },
                    ..Default::default()
                })
                .insert( Projectile{})
                .insert(Friendly{})
                .insert(Damage::new(100.0))
                .insert(BoxCollider::new(190.0*weapon.scale, 420.0*weapon.scale))
                .insert(Lifetime::new(weapon.projectile_lifetime))
                .insert(Moveable::new(true, weapon.projectile_speed, rotation.mul_vec3(Vec3::Y)));
        }
    }
}


pub fn spawn_w_meteor(mut commands: Commands) {
    commands
        .spawn()
        .insert(Weapon { 
            texture: "meteor.png".to_string(), 
            scale: 0.1,

            cooldown: Duration::from_secs_f32(0.1), 
            cooldown_timer: Duration::from_secs_f32(0.0),
            projectile_speed: 6.0,
            projectile_lifetime: 1.5,
        });
}