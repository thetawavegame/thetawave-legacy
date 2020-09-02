use crate::{
    components::{Hitbox2DComponent, Item},
    resources::{ItemEntityData, SpriteResource},
};
use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{Builder, Entities, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent},
};

pub fn spawn_item(
    entities: &Entities,
    item_resource: &ReadExpect<SpriteResource>,
    item: ItemEntityData,
    spawn_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let sprite_render = SpriteRender {
        sprite_sheet: item_resource.items_sprite_sheet.clone(),
        sprite_number: item.item_component.sprite_index,
    };
    //super::spawn_sprite_entity(&entities, sprite, item, spawn_position, &lazy_update);
    let mut local_transform = Transform::default();
    local_transform.set_translation(spawn_position);

    println!("{} spawned!", item.item_component.name);

    lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(item.item_component)
        .with(item.hitbox_component)
        .with(local_transform)
        .with(Transparent)
        .build();
}
