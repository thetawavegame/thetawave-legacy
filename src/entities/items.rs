use crate::resources::{ItemEntityData, ItemsResource, SpriteSheetsResource};
use amethyst::{
    core::{math::Vector3, transform::Transform, Named},
    ecs::prelude::{Builder, Entities, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent},
};

pub fn spawn_item(
    entities: &Entities,
    spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
    item: ItemEntityData,
    items_resource: &ReadExpect<ItemsResource>,
    spawn_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let sprite_render = SpriteRender {
        sprite_sheet: spritesheets_resource.spritesheets["items"].clone(),
        sprite_number: item.item_component.sprite_index,
    };
    let mut local_transform = Transform::default();
    local_transform.set_translation(spawn_position);

    let name = Named::new("item");

    println!("{} spawned!", item.item_component.name);

    let item_entity = lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(item.item_component)
        .with(items_resource.hitbox2d_component.clone())
        .with(items_resource.motion2d_component.clone())
        .with(local_transform)
        .with(Transparent)
        .with(name)
        .with(items_resource.despawn_border_component.clone())
        .build();

    if let Some(animation_component) = item.animation_component {
        lazy_update.insert(item_entity, animation_component);
    }
}
