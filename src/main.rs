#![windows_subsystem = "windows"]

extern crate amethyst;

use amethyst::{
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, ALPHA, ColorMask},
    utils::application_root_dir,
    ui::{DrawUi},
    core::transform::TransformBundle,
    input::InputBundle,
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
    log.level_filter = amethyst::LogLevelFilter::Warn;
    amethyst::start_logger(log);

    let path = format!(
        "{}/resources/display_config_960.ron",
        application_root_dir()
    );

    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build()
        .with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new().with_transparency(ColorMask::all(), ALPHA, None))
            .with_pass(DrawUi::new()),
    );

    let bindings_path = format!(
        "{}/resources/bindings_config.ron", application_root_dir(),
    );

    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(bindings_path)?;

    let game_data =
        GameDataBuilder::default()
            .with_bundle(RenderBundle::new(pipe, Some(config))
            .with_sprite_sheet_processor()
            .with_sprite_visibility_sorting(&[]))?
            .with_bundle(TransformBundle::new())?
            .with_bundle(input_bundle)?;

    let mut game = Application::new("./", SpaceShooter::default(), game_data)?;

    game.run();

    Ok(())
}
