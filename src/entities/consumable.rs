use amethyst::{
    ecs::prelude::{Entities, LazyUpdate, ReadExpect},
    renderer::{SpriteRender},
    core::math::Vector3
};
use crate::{
    resources::SpriteResource,
    components::Consumable,
};

pub fn spawn_consumable(entities: &Entities, item_resource: &ReadExpect<SpriteResource>, item: Consumable, spawn_position: Vector3<f32>, lazy_update: &ReadExpect<LazyUpdate>) {                                    
    let sprite = SpriteRender {
        sprite_sheet: item_resource.consumables_sprite_sheet.clone(),
        sprite_number: item.sprite_index,
    };
    super::spawn_sprite_entity(&entities, sprite, item, spawn_position, &lazy_update);
}