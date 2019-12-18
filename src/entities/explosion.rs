use amethyst::{
    ecs::prelude::{Entities, LazyUpdate, ReadExpect},
    renderer::{SpriteRender},
    core::math::Vector3
};
use crate::{
    resources::SpriteResource,
    components::{Explosion, Animation, AnimationType},
};

pub fn spawn_explosion(entities: &Entities, sprite_resource: &ReadExpect<SpriteResource>, sprite_number: usize, spawn_position: Vector3<f32>, lazy_update: &ReadExpect<LazyUpdate>) {
    let frame_time: f32 = 0.1;
    let frame_count: usize = 7;
    let duration: f32 = frame_time * (frame_count - 1) as f32;

    let sprite = SpriteRender {
        sprite_sheet: sprite_resource.explosions_sprite_sheet.clone(),
        sprite_number,
    };
    
    let animation = Animation {
        start_idx: 0,
        frame_count: frame_count,
        current_frame: 0,
        frame_time: frame_time,
        elapsed_time: 0.0,
        forward: true,
        animation_type: AnimationType::Forward
    };

    super::spawn_animated_entity(&entities, sprite, animation, Explosion{ duration, name: "explosion".to_string()}, spawn_position, &lazy_update);
}