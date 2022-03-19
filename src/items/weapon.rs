use bevy::prelude::*;
use bevy::utils::Duration;

use crate::player::Player;
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

pub fn update_weapons_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut wep_query: Query<&mut Weapon>,
        player_query: Query<(&Player, &Transform, &TextureAtlasSprite)>,
    time: Res<Time>,

){
    let (player, player_transform, sprite) = player_query.get_single().unwrap();
    for mut weapon in wep_query.iter_mut() {
        weapon.cooldown_timer += time.delta();
        if weapon.cooldown_timer > weapon.cooldown {
            weapon.cooldown_timer = Duration::from_secs_f32(0.0);
            let mut flip = 1.0;
            if sprite.flip_x { flip = -1.0; }
            commands
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load(&weapon.texture),
                    transform: Transform {
                        translation: player_transform.translation,
                        scale: Vec3::new(1.0, 1.0, 1.0) * weapon.scale,
                        rotation: Quat::from_rotation_z(-player.get_direction().angle_between(Vec3::Y) * flip),
                    },
                    ..Default::default()
                })
                .insert( Projectile::new(weapon.projectile_speed, weapon.projectile_lifetime));
        }
    }
}




pub fn spawn_w_meteor(mut commands: Commands) {
    commands
        .spawn()
        .insert(Weapon { 
            texture: "meteor.png".to_string(), 
            scale: 0.2,

            cooldown: Duration::from_secs_f32(1.0), 
            cooldown_timer: Duration::from_secs_f32(0.0),
            projectile_speed: 10.0,
            projectile_lifetime: 1.5,
        });
}