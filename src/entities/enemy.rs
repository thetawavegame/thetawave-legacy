use amethyst::{
    ecs::prelude::{Entities, LazyUpdate, ReadExpect},
    renderer::{SpriteRender},
    core::math::Vector3
};
use crate::{
    resources::SpriteResource,
    components::{Enemy, Animation},
};

pub fn spawn_enemy(entities: &Entities, item_resource: &ReadExpect<SpriteResource>, item: Enemy, spawn_position: Vector3<f32>, lazy_update: &ReadExpect<LazyUpdate>) {                                    
    let sprite = SpriteRender {
        sprite_sheet: item_resource.enemy_animations_sprite_sheet.clone(),
        sprite_number: item.sprite_index,
    };

    let animation = Animation {
        start_idx: item.sprite_index,
        frame_count: 3,
        current_frame: item.sprite_index,
        frame_time: 0.18,
        elapsed_time: 0.0,
        forward: true,
    };

    super::spawn_enemy_entity(&entities, sprite, animation, item, spawn_position, &lazy_update);

}