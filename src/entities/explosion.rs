use amethyst::{
    ecs::prelude::{Entities, LazyUpdate, ReadExpect},
    renderer::{SpriteRender},
    core::math::Vector3
};
use crate::{
    resources::SpriteResource,
    components::Explosion,
};

const EXPLOSION_DURATION: f32 = 0.3;

pub fn spawn_explosion(entities: &Entities, item_resource: &ReadExpect<SpriteResource>, sprite_number: usize, spawn_position: Vector3<f32>, lazy_update: &ReadExpect<LazyUpdate>) {
    let sprite = SpriteRender {
        sprite_sheet: item_resource.explosions_sprite_sheet.clone(),
        sprite_number,
    };
    super::spawn_sprite_entity(&entities, sprite, Explosion{ duration: EXPLOSION_DURATION}, spawn_position, &lazy_update);
}