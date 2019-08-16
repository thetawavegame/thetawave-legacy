use amethyst::{
    prelude::Builder,
    ecs::prelude::World,
    core::transform::Transform,
    renderer::{SpriteRender, SpriteSheet},
    assets::Handle,
};

use crate::{
    space_shooter::{ARENA_MAX_X, ARENA_HEIGHT}
};

const WIDTH: f32 = 45.0;
const Z: f32 = 0.9;
const LEFT_SPRITE_INDEX: usize = 0;
const RIGHT_SPRITE_INDEX: usize = 1;

pub fn initialise_side_panels(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {

    let sprite_render_left = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: LEFT_SPRITE_INDEX,
    };

    let sprite_render_right = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: RIGHT_SPRITE_INDEX,
    };


    /*
    let (screen_width, screen_height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width()+(dim.width()*(1.0/6.0)), dim.height()+(dim.height()*(1.0/6.0)))
    };
    println!("screen width: {} screen height: {}", screen_width, screen_height);
*/

    let mut local_transform_left = Transform::default();
    local_transform_left.set_translation_xyz(WIDTH/2.0, (ARENA_HEIGHT/2.0) - 1.0, Z);

    let mut local_transform_right = Transform::default();
    local_transform_right.set_translation_xyz(ARENA_MAX_X + (WIDTH/2.0), (ARENA_HEIGHT/2.0) - 1.0, Z);

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