use amethyst::{
    prelude::Builder,
    ecs::prelude::World,
    core::transform::Transform,
    renderer::{PngFormat, Texture, TextureMetadata, ScreenDimensions},
    assets::{AssetStorage, Loader},
};

use crate::{
    space_shooter::{ARENA_MAX_X, ARENA_HEIGHT}
};


pub fn initialise_side_panels(world: &mut World) {

    let image_left = {
        let loader = world.read_resource::<Loader>();
        let side_panel_image = loader.load(
            "texture/side_panel_metal.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        );
        side_panel_image
    };

    let image_right = {
        let loader = world.read_resource::<Loader>();
        let side_panel_image = loader.load(
            "texture/side_panel_metal_right.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        );
        side_panel_image
    };

    let (screen_width, screen_height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width()+(dim.width()*(1.0/6.0)), dim.height()+(dim.height()*(1.0/6.0)))
    };
    println!("screen width: {} screen height: {}", screen_width, screen_height);


    let mut local_transform_left = Transform::default();
    local_transform_left.set_xyz((45.0/2.0)- 1.0, (ARENA_HEIGHT/2.0) - 1.0, 9.0);

    let mut local_transform_right = Transform::default();
    local_transform_right.set_xyz(ARENA_MAX_X + (45.0/2.0)- 1.0, (ARENA_HEIGHT/2.0) - 1.0, 9.0);

    world
        .create_entity()
        .with(local_transform_left)
        .with(image_left)
        .build();

    world
        .create_entity()
        .with(local_transform_right)
        .with(image_right)
        .build();
}