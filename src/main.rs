#![windows_subsystem = "windows"]

extern crate amethyst;

use amethyst::{
    prelude::*,
    renderer::{
        RenderingBundle,
        types::DefaultBackend,
        plugins::{RenderFlat2D, RenderToWindow},
    },
    utils::application_root_dir,
    core::{transform::TransformBundle},
    input::{InputBundle, StringBindings},
};

mod space_shooter;
pub mod systems;
pub mod components;
pub mod resources;
pub mod entities;

use crate::space_shooter::SpaceShooter;


fn main() -> amethyst::Result<()> {

    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display_config_960.ron");
    let bindings_path = app_root.join("config").join("bindings_config.ron");
    let assets_path = app_root.join("assets");

    let game_data = GameDataBuilder::default()
            .with_bundle(TransformBundle::new())?
            .with_bundle(
                InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_path)?,
            )?
            .with_bundle(
                RenderingBundle::<DefaultBackend>::new()
                    .with_plugin(
                        RenderToWindow::from_config_path(display_config_path)
                            .with_clear([0.0, 0.0, 0.0, 1.0]),
                    )
                    .with_plugin(RenderFlat2D::default())
            )?;
    
    let mut game = Application::new(assets_path, SpaceShooter::default(), game_data)?;

    game.run();

    Ok(())
}
