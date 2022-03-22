use bevy::prelude::*;

use crate::components::Moveable;



// Animate spritesheet system
pub fn animate_spritesheet(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&Moveable, &mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {

    for (moveable, mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {

        let facing = moveable.get_direction().x;
        if facing < 0.0 {
            sprite.flip_x = true;
        } else if facing > 0.0 {
            sprite.flip_x = false;
        }

        if moveable.is_moving() {
            timer.tick(time.delta());
            if timer.finished() {
                let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            }
        } else {
            sprite.index = 0;
        }
    }
}