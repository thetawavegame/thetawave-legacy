use amethyst::{
    prelude::Builder,
    ecs::prelude::World,
    core::transform::Transform,
    renderer::{PngFormat, Texture, TextureMetadata, ScreenDimensions},
    ui::{UiImage, UiTransform, Anchor},
    assets::{AssetStorage, Loader},
};

use crate::{
    components::SidePanel,
};


pub fn initialise_side_panels(world: &mut World) {

    let image = {
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


    let local_transform_left = UiTransform::new("Left".to_string(), Anchor::TopLeft, ((screen_width/8.0)/2.0), (screen_height/2.0), 0.0,screen_width/8.0, screen_height, 0);
    let local_transform_right = UiTransform::new("Right".to_string(), Anchor::TopRight, (screen_width-((screen_width/8.0)/2.0)), (screen_height/2.0), 0.0,screen_width/8.0, screen_height, 0);


    world
        .create_entity()
        .with(local_transform_left)
        .with(UiImage {
            texture: image.clone(),
        })
        .with(SidePanel{})
        .build();

    world
        .create_entity()
        .with(local_transform_right)
        .with(UiImage {
            texture: image_right.clone(),
        })
        .with(SidePanel{})
        .build();
}