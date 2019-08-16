#![windows_subsystem = "windows"]

extern crate amethyst;

use amethyst::{
    //rendy::sprite_visibility::SpriteVisibilitySortingSystem,
    assets::Processor,
    prelude::*,
    renderer::{
        RenderingBundle,
        SpriteSheet,
        types::DefaultBackend,
        plugins::{RenderFlat2D, RenderToWindow},
        sprite_visibility::SpriteVisibilitySortingSystem,
    },
    window::{DisplayConfig},
    utils::application_root_dir,
    ui::{RenderUi, UiBundle, DrawUi},
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
};

mod space_shooter;
pub mod systems;
pub mod components;
pub mod resources;
pub mod entities;

use crate::space_shooter::SpaceShooter;


fn main() -> amethyst::Result<()> {
    //start logging
    let mut log = amethyst::LoggerConfig::default();
    //log.level_filter = amethyst::LogLevelFilter::Warn;
    amethyst::start_logger(log);

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("resources/display_config_960.ron");

/*
    let pipe = Pipeline::build()
        .with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new().with_transparency(ColorMask::all(), ALPHA, None))
            .with_pass(DrawUi::new()),
    );
*/

    let bindings_path = app_root.join("resources/bindings_config.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(bindings_path)?;

    let game_data =
        GameDataBuilder::default()
            //.with_bundle(RenderBundle::new(pipe, Some(config))
            .with_bundle(
                RenderingBundle::<DefaultBackend>::new()
                    .with_plugin(
                        RenderToWindow::from_config_path(display_config_path)
                            .with_clear([0.0, 0.0, 0.0, 1.0]),
                    )
                    .with_plugin(RenderFlat2D::default())
                    .with_plugin(RenderUi::default()),
            )?
            //.with_sprite_sheet_processor()
            .with(
                Processor::<SpriteSheet>::new(),
                "sprite_sheet_processor",
                &[],
            )
            .with(
                SpriteVisibilitySortingSystem::new(),
                "sprite_visibility_system",
                &["transform_system"]
            )
            .with_bundle(TransformBundle::new())?
            .with_bundle(input_bundle)?;

    let mut game = Application::new("./", SpaceShooter::default(), game_data)?;

    game.run();

    Ok(())
}
