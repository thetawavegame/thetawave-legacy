use amethyst::{
    prelude::Builder,
    ecs::prelude::World,
    core::transform::Transform,
    renderer::{PngFormat, Texture, TextureMetadata, ScreenDimensions},
    ui::{UiImage, UiTransform, Anchor},
    assets::{AssetStorage, Loader},
};

use crate::{
    components::HealthBar,
    entities::spawn_health_unit,
};


pub fn initialise_health_bar(world: &mut World) {

    let image = {
        let loader = world.read_resource::<Loader>();
        let side_panel_image = loader.load(
            "texture/health_bar.png",
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

    world
        .create_entity()
        .with(HealthBar{
            x_pos: 332.0,
            y_pos: 200.0,
            health_stack: vec![],
        })
        .build();
}