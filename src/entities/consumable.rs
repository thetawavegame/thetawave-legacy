use crate::resources::{ConsumableEntityData, SpriteSheets};
use amethyst::{
    core::{math::Vector3, transform::Transform, Named},
    ecs::prelude::{Builder, Entities, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent},
};

pub fn spawn_consumable(
    entities: &Entities,
    sprite_resource: &ReadExpect<SpriteSheets>,
    consumable: ConsumableEntityData,
    spawn_position: &Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_resource.spritesheets["consumables"].clone(),
        sprite_number: consumable.consumable_component.sprite_index,
    };
    let name = Named::new("consumable");

    let mut local_transform = Transform::default();
    local_transform.set_translation(*spawn_position);

    lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(consumable.hitbox_component)
        .with(consumable.consumable_component)
        .with(local_transform)
        .with(Transparent)
        .with(name)
        .build();
}
