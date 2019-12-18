use amethyst::{
    ecs::prelude::{Entities, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, SpriteSheet},
    core::math::Vector3,
    assets::Handle,
};
use crate::{
    components::{Enemy, Animation},
};

pub fn spawn_enemy(entities: &Entities, sprite_sheet: Handle<SpriteSheet>, item: Enemy, spawn_position: Vector3<f32>, lazy_update: &ReadExpect<LazyUpdate>) {
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet,
        sprite_number: item.sprite_index,
    };

    let animation = Animation {
        start_idx: item.sprite_index,
        frame_count: item.frame_count,
        current_frame: item.sprite_index,
        frame_time: item.frame_time,
        elapsed_time: 0.0,
        forward: true,
        animation_type: item.animation_type.clone(),
    };

    super::spawn_animated_entity(&entities, sprite, animation, item, spawn_position, &lazy_update);

}