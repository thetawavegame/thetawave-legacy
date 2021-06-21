use crate::{
    constants::{ARENA_MAX_X, ARENA_MIN_Y},
    visual::components::StoreIconComponent,
};
use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::{World, WorldExt},
    prelude::Builder,
    renderer::{SpriteRender, SpriteSheet},
};

pub fn initialize_store_icons(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let store_icon_1 = StoreIconComponent { inventory_index: 0 };
    let store_icon_2 = StoreIconComponent { inventory_index: 1 };
    let store_icon_3 = StoreIconComponent { inventory_index: 2 };

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0, // index of the blank item icon on the spritesheet
    };

    let mut transform_1 = Transform::default();
    transform_1.set_translation_xyz(ARENA_MAX_X + 12.0, ARENA_MIN_Y + 72.0, 0.9);

    let mut transform_2 = transform_1.clone();
    transform_2.set_translation_y(ARENA_MIN_Y + 53.0);

    let mut transform_3 = transform_1.clone();
    transform_3.set_translation_y(ARENA_MIN_Y + 34.0);

    world
        .create_entity()
        .with(store_icon_1)
        .with(transform_1)
        .with(sprite_render.clone())
        .build();

    world
        .create_entity()
        .with(store_icon_2)
        .with(transform_2)
        .with(sprite_render.clone())
        .build();

    world
        .create_entity()
        .with(store_icon_3)
        .with(transform_3)
        .with(sprite_render)
        .build();
}
