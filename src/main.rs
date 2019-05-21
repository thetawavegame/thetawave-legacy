extern crate amethyst;

use amethyst::{
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    utils::application_root_dir,
    ui::{DrawUi},
    core::transform::TransformBundle,
    input::InputBundle,
};

mod space_shooter;
mod systems;

pub mod components;
pub mod resources;
pub mod entities;
use crate::space_shooter::SpaceShooter;


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!(
        "{}/resources/display_config.ron",
        application_root_dir()
    );

    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
            .with_pass(DrawUi::new()),
    );

    let bindings_path = format!(
        "{}/resources/bindings_config.ron", application_root_dir(),
    );

    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(bindings_path)?;

    let game_data =
        GameDataBuilder::default().with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?.with_bundle(
            TransformBundle::new())?.with_bundle(input_bundle)?.with(systems::SpaceshipSystem, "spaceship_system", &[])
            .with(systems::BlastSystem, "blast_system", &[]);

    let mut game = Application::new("./", SpaceShooter, game_data)?;

    game.run();

    Ok(())
}
