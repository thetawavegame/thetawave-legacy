use amethyst::{
    ecs::prelude::{Entities, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, SpriteSheet},
    core::math::Vector3,
    assets::Handle,
};
use crate::{
    resources::SpriteResource,
    components::{Enemy, Animation, AnimationType},
};

pub fn spawn_enemy(entities: &Entities, sprite_sheet: Handle<SpriteSheet>, item: Enemy, spawn_position: Vector3<f32>, lazy_update: &ReadExpect<LazyUpdate>) {
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet,
        sprite_number: item.sprite_index,
    };

    let animation = Animation {
        start_idx: item.sprite_index,
        frame_count: 3, // TODO: Replace with variable
        current_frame: item.sprite_index,
        frame_time: 0.18, // TODO: Replace with variable
        elapsed_time: 0.0,
        forward: true,
        animation_type: AnimationType::PingPong,
    };

    super::spawn_animated_entity(&entities, sprite, animation, item, spawn_position, &lazy_update);

}