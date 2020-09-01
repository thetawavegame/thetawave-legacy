use crate::{
    components::{Hitbox2DComponent, Item},
    resources::SpriteResource,
};
use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{Builder, Entities, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent},
};

pub fn spawn_item(
    entities: &Entities,
    item_resource: &ReadExpect<SpriteResource>,
    item: Item,
    spawn_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let sprite_render = SpriteRender {
        sprite_sheet: item_resource.items_sprite_sheet.clone(),
        sprite_number: item.sprite_index,
    };
    //super::spawn_sprite_entity(&entities, sprite, item, spawn_position, &lazy_update);
    let mut local_transform = Transform::default();
    local_transform.set_translation(spawn_position);

    let hitbox = Hitbox2DComponent {
        width: 14.0,
        height: 14.0,
        offset_x: 0.0,
        offset_y: 0.0,
        offset_rotation: 0.0,
    };

    println!("{} spawned!", item.name);

    lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(item)
        .with(hitbox)
        .with(local_transform)
        .with(Transparent)
        .build();
}
