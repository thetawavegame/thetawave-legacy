//#![windows_subsystem = "windows"]
extern crate amethyst;
extern crate serde;

use amethyst::{
    audio::AudioBundle,
    core::transform::TransformBundle,
    gltf::GltfSceneLoaderSystemDesc,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderFlat3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod audio;
pub mod components;
pub mod constants;
pub mod entities;
pub mod resources;
mod space_shooter;
pub mod systems;

use crate::space_shooter::SpaceShooter;
use resources::{ConsumablePool, EnemyPool, ItemPool};

use amethyst::config::Config;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display_fullscreen.ron");
    let bindings_path = app_root.join("config").join("bindings_config.ron");
    let assets_path = app_root.join("assets");

    let items = <ItemPool as Config>::load_no_fallback(assets_path.join("data").join("items.ron"))
        .expect("failed to load game data");
    let enemies =
        <EnemyPool as Config>::load_no_fallback(assets_path.join("data").join("enemies.ron"))
            .expect("failed to load game data");
    let consumables = <ConsumablePool as Config>::load_no_fallback(
        assets_path.join("data").join("consumables.ron"),
    )
    .expect("failed to load game data");

    let game_data = GameDataBuilder::default()
        .with_system_desc(GltfSceneLoaderSystemDesc::default(), "gltf_system", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_path)?)?
        .with_bundle(AudioBundle::default())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat3D::default())
                .with_plugin(RenderFlat2D::default())
                //.with_plugin(RenderShaded3D::default())
                .with_plugin(RenderUi::default()),
        )?;

    let mut game = Application::build(assets_path, SpaceShooter::default())?
        .with_resource(items)
        .with_resource(enemies)
        .with_resource(consumables)
        .build(game_data)?;

    game.run();

    Ok(())
}
