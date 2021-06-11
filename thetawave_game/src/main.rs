#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]

extern crate amethyst;
extern crate thetawave_lib;

use amethyst::{
    audio::AudioBundle,
    core::transform::TransformBundle,
    gltf::GltfSceneLoaderSystemDesc,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderDebugLines, RenderFlat2D, RenderFlat3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod data_include;
mod states;

use states::MainGameState;
use thetawave_lib::{
    resources::{
        ConsumableModifiersResource, DebugLinesConfig, DefenseResource, DropTablesResource,
        GameParametersResource, ItemModifiersResource, PhaseManagerResource, PlayersResource,
        SoundsConfig, SpriteSheetsConfig, StoreResource,
    },
    spawn::resources::SpawnerResource,
    spawnable::resources::{ConsumablesResource, EffectsResource, ItemsResource, MobsResource},
};

use amethyst::config::Config;

use data_include::{generate_configs, load_include_data, IncludeData};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    generate_configs();

    let app_root = application_root_dir()?;
    let config_path = app_root.join("config");

    let display_config_path = config_path.join("display_config_960.ron");
    //let display_config_path = config_path.join("display_fullscreen.ron");
    let bindings_path = config_path.join("bindings_config.ron");

    let debug_lines =
        <DebugLinesConfig as Config>::load(config_path.join("debug_lines_config.ron"))
            .expect("failed to load configuration file: debug_lines_config.ron");
    let spritesheets =
        <SpriteSheetsConfig as Config>::load(config_path.join("spritesheets_config.ron"))
            .expect("failed to load configuration file: spritesheets_config.ron");
    let sounds = <SoundsConfig as Config>::load(config_path.join("sounds_config.ron"))
        .expect("failed to load configuration file: sounds_config.ron");

    let IncludeData {
        items,
        item_modifiers,
        consumable_modifiers,
        mobs,
        consumables,
        effects,
        players,
        phases,
        store,
        game_parameters,
        spawner,
        defense,
        drop_tables,
    } = load_include_data();

    let items =
        <ItemsResource as Config>::load_bytes(items).expect("failed to load data file: items.ron");
    let item_modifiers = <ItemModifiersResource as Config>::load_bytes(item_modifiers)
        .expect("failed to load data file: item_modifiers.ron");
    let consumable_modifiers =
        <ConsumableModifiersResource as Config>::load_bytes(consumable_modifiers)
            .expect("failed to load data file: consumable_modifiers.ron");
    let mobs =
        <MobsResource as Config>::load_bytes(mobs).expect("failed to load data file: mobs.ron");
    let consumables = <ConsumablesResource as Config>::load_bytes(consumables)
        .expect("failed to load data file: consumables.ron");
    let effects = <EffectsResource as Config>::load_bytes(effects)
        .expect("failed to load data file: effects.ron");
    let players = <PlayersResource as Config>::load_bytes(players)
        .expect("failed to load data file: players.ron");
    let phases = <PhaseManagerResource as Config>::load_bytes(phases)
        .expect("failed to load data file: phases.ron");
    let store =
        <StoreResource as Config>::load_bytes(store).expect("failed to load data file: store.ron");
    let game_parameters = <GameParametersResource as Config>::load_bytes(game_parameters)
        .expect("failed to load data file: game_parameters.ron");
    let spawner = <SpawnerResource as Config>::load_bytes(spawner)
        .expect("failed to load data file: spawner.ron");
    let defense = <DefenseResource as Config>::load_bytes(defense)
        .expect("failed to load data file: defense.ron");
    let drop_tables = <DropTablesResource as Config>::load_bytes(drop_tables)
        .expect("failed to load data file: drop_tables.ron");

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
                .with_plugin(RenderUi::default())
                .with_plugin(RenderDebugLines::default()),
        )?;

    let mut game = Application::build(app_root.join("assets"), MainGameState::default())?
        .with_resource(items)
        .with_resource(item_modifiers)
        .with_resource(consumable_modifiers)
        .with_resource(mobs)
        .with_resource(consumables)
        .with_resource(effects)
        .with_resource(players)
        .with_resource(spritesheets)
        .with_resource(sounds)
        .with_resource(debug_lines)
        .with_resource(phases)
        .with_resource(store)
        .with_resource(game_parameters)
        .with_resource(spawner)
        .with_resource(defense)
        .with_resource(drop_tables)
        .build(game_data)?;

    game.run();

    Ok(())
}
