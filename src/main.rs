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
            .with_bundle(TransformBundle::new())?.with_bundle(input_bundle)?
            .with(systems::SpaceshipSystem, "spaceship_system", &[])
            .with(systems::BlastSystem, "blast_system", &[])
            .with(systems::EnemySystem, "enemy_system", &[])
            .with(systems::SpawnerSystem, "spawner_system", &[])
            .with(systems::PlayerHitSystem, "player_hit_system", &[])
            .with(systems::ExplosionSystem, "explosion_system", &[])
            .with(systems::ItemSystem, "item_system", &[])
            .with(systems::BarrelRollSystem, "barrel_roll_system", &[])
            .with(systems::SpaceshipMovementSystem, "spaceship_movement_system", &[])
            .with(systems::ItemSpawnSystem, "item_spawn_system", &[]);

    let mut game = Application::new("./", SpaceShooter, game_data)?;

    game.run();

    Ok(())
}
