use bevy::prelude::*;
use bevy::utils::Duration;

use crate::components::{Moveable, Friendly, Damage, BoxCollider, Lifetime, Playable};
use super::projectile::Projectile;

#[derive(Component)]
pub struct Gun {
    proj_size: Vec2,
    proj_scale: f32,
    proj_texture: String,
    proj_speed: f32,
    proj_lifetime: Duration,
    timer: Timer,
}

pub fn update_guns(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut gun_query: Query<&mut Gun>,
    player_query: Query<(&Transform, &TextureAtlasSprite, &Moveable), With<Playable>>,
    time: Res<Time>,
){
    let (player_transform, sprite, moveable) = player_query.get_single().unwrap();

    for mut gun in gun_query.iter_mut() {
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
                .insert(Projectile{})
                .insert(Friendly{})
                .insert(Damage::new(100.0))
                .insert(BoxCollider::new(gun.proj_size * gun.proj_scale))
                .insert(Lifetime::new(gun.proj_lifetime))
                .insert(Moveable::new(true, gun.proj_speed, rotation.mul_vec3(Vec3::Y)));
        }
    }
}


pub fn spawn_w_meteor(
    In(player): In<Entity>,
    mut commands: Commands
) {
    commands
        .spawn()
        .insert(meteor_blaster())
        .insert(Parent(player));
}

fn meteor_blaster() -> Gun {
    Gun {
        proj_size: Vec2::new(190.0, 420.0),
        proj_scale: 0.1,
        proj_texture: "meteor.png".to_string(),
        proj_speed: 10.0,
        proj_lifetime: Duration::from_secs_f32(1.5),
        timer: Timer::from_seconds(0.5, true),
    }
}