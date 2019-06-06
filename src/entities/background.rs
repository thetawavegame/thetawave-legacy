use amethyst::{
    prelude::Builder,
    ecs::prelude::World,
    core::transform::Transform,
    renderer::{SpriteRender, SpriteSheetHandle, Transparent, PngFormat, Texture, TextureMetadata, ScreenDimensions},
    ui::{UiImage, UiTransform, Anchor},
    assets::{AssetStorage, Loader},
};

use crate::{
    components::Background,
    space_shooter::{GAME_WIDTH, GAME_HEIGHT, ARENA_MAX_X, ARENA_MAX_Y, ARENA_MIN_X, ARENA_MIN_Y, ARENA_WIDTH, ARENA_HEIGHT},
};


pub fn initialise_background(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {


    let mut local_transform = Transform::default();
    //local_transform.set_xyz(GAME_WIDTH / 2.0, GAME_HEIGHT / 6.0, 0.9);
    local_transform.set_xyz(ARENA_MIN_X + (ARENA_WIDTH/2.0), ARENA_HEIGHT/2.0, -5.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Background {})
        .with(local_transform)
        .with(Transparent)
        .build();
}