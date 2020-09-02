use crate::{components::Consumable, resources::SpriteResource};
use amethyst::{
    core::{math::Vector3, Named},
    ecs::prelude::{Entities, LazyUpdate, ReadExpect},
    renderer::SpriteRender,
};

pub fn spawn_consumable(
    entities: &Entities,
    sprite_resource: &ReadExpect<SpriteResource>,
    item: Consumable,
    spawn_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let sprite = SpriteRender {
        sprite_sheet: sprite_resource.consumables_sprite_sheet.clone(),
        sprite_number: item.sprite_index,
    };
    let name = Named::new("consumable");

    super::spawn_sprite_entity(&entities, name, sprite, item, spawn_position, &lazy_update);
}
