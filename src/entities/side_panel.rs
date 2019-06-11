use amethyst::{
    prelude::Builder,
    ecs::prelude::World,
    core::transform::Transform,
    renderer::{ScreenDimensions, SpriteRender, SpriteSheetHandle},
};

use crate::{
    space_shooter::{ARENA_MAX_X, ARENA_HEIGHT}
};


pub fn initialise_side_panels(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {

    let sprite_render_left = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    let sprite_render_right = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 1,
    };


    let (screen_width, screen_height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width()+(dim.width()*(1.0/6.0)), dim.height()+(dim.height()*(1.0/6.0)))
    };
    println!("screen width: {} screen height: {}", screen_width, screen_height);


    let mut local_transform_left = Transform::default();
    local_transform_left.set_xyz(45.0/2.0, (ARENA_HEIGHT/2.0) - 1.0, 0.8);

    let mut local_transform_right = Transform::default();
    local_transform_right.set_xyz(ARENA_MAX_X + (45.0/2.0), (ARENA_HEIGHT/2.0) - 1.0, 0.8);

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