use amethyst::{
    prelude::Builder,
    ecs::prelude::World,
    core::transform::Transform,
    renderer::{SpriteRender, SpriteSheet, Transparent},
    assets::Handle,
};
use crate::{
    space_shooter::{ARENA_MIN_X, ARENA_WIDTH, ARENA_HEIGHT},
};

const Z: f32 = 0.0;

pub fn initialise_background(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {

    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_MIN_X + (ARENA_WIDTH/2.0), ARENA_HEIGHT/2.0, Z);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(local_transform)
        .with(Transparent)
        .build();
}