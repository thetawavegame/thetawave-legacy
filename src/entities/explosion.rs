use amethyst::{
    ecs::prelude::{Entities, LazyUpdate, ReadExpect},
    renderer::{SpriteRender},
    core::math::Vector3
};
use crate::{
    resources::SpriteResource,
    components::Explosion,
    constants::EXPLOSION_DURATION,
};

pub fn spawn_explosion(entities: &Entities, sprite_resource: &ReadExpect<SpriteResource>, sprite_number: usize, spawn_position: Vector3<f32>, lazy_update: &ReadExpect<LazyUpdate>) {
    let sprite = SpriteRender {
        sprite_sheet: sprite_resource.explosions_sprite_sheet.clone(),
        sprite_number,
    };
    super::spawn_sprite_entity(&entities, sprite, Explosion{ duration: EXPLOSION_DURATION, name: "explosion".to_string()}, spawn_position, &lazy_update);
}