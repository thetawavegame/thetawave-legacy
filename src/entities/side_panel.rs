use amethyst::{
    prelude::Builder,
    ecs::{World, WorldExt},
    core::transform::Transform,
    renderer::{SpriteRender, SpriteSheet},
    assets::{Handle},
};
use crate::{
    constants::{
        ARENA_MAX_X, ARENA_HEIGHT, SIDE_PANEL_WIDTH, SIDE_PANEL_Z, SIDE_PANEL_LEFT_SPRITE_INDEX,
        SIDE_PANEL_RIGHT_SPRITE_INDEX,
    },
};

pub fn initialise_side_panels(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {

    let sprite_render_left = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: SIDE_PANEL_LEFT_SPRITE_INDEX,
    };

    let sprite_render_right = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: SIDE_PANEL_RIGHT_SPRITE_INDEX,
    };

    let mut local_transform_left = Transform::default();
    local_transform_left.set_translation_xyz(SIDE_PANEL_WIDTH/2.0, (ARENA_HEIGHT/2.0) - 1.0, SIDE_PANEL_Z);

    let mut local_transform_right = Transform::default();
    local_transform_right.set_translation_xyz(ARENA_MAX_X + (SIDE_PANEL_WIDTH/2.0), (ARENA_HEIGHT/2.0) - 1.0, SIDE_PANEL_Z);

    world
        .create_entity()
        .with(local_transform_left)
        .with(sprite_render_left)
        .build();

    world
        .create_entity()
        .with(local_transform_right)
        .with(sprite_render_right)
        .build();
}