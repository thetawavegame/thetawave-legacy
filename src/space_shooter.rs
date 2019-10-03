use amethyst::{
    assets::{
        AssetLoaderSystemData, AssetStorage, Loader, Handle, PrefabLoader,
        PrefabLoaderSystemDesc, RonFormat, Prefab, AssetPrefab
    },
    gltf::{GltfSceneAsset, GltfSceneFormat, GltfSceneLoaderSystemDesc, GltfPrefab},
    core::transform::{Transform},
    prelude::*,
    renderer::{
        Camera, SpriteSheet, SpriteSheetFormat, Texture, SpriteRender, camera, shape::Shape,
        Material, Mesh,
    },
    input::{is_key_down, VirtualKeyCode},
    ecs::prelude::{Dispatcher, DispatcherBuilder, Entity},
    renderer::{
        formats::texture::ImageFormat,
        rendy::mesh::{Normal, Position, TexCoord, PosNormTangTex},
    },
    ui::{TtfFormat, Anchor, UiText, UiTransform},
    utils::{scene::BasicScenePrefab}
};
use crate::{
    audio::initialise_audio,
    systems,
    entities::{
        initialise_gamemaster, initialise_spaceship, initialise_enemy_spawner,
        initialise_side_panels, initialise_background, initialise_defense,
        initialise_status_bars, initialise_store, initialise_planet
    },
    resources::{initialise_sprite_resource},
    constants::{
        GAME_WIDTH, GAME_HEIGHT, ARENA_WIDTH, ARENA_HEIGHT, ARENA_MIN_X, ARENA_MAX_X,
        ARENA_MIN_Y, ARENA_MAX_Y,
    },
};
use std::fs::File;
use std::f32::consts::{FRAC_PI_3};

#[derive(Debug)]
pub struct CollisionEvent {
    pub entity_a: Entity,
    pub type_a: String,
    pub to_velocity_x_a: f32, //velocity of the entity acting on a
    pub to_velocity_y_a: f32, 
    pub entity_b: Entity,
    pub type_b: String,
    pub to_velocity_x_b: f32, //velocity of the entity acting on b
    pub to_velocity_y_b: f32,
}

impl CollisionEvent {
    pub fn new(entity_a: Entity, type_a: String, to_velocity_x_a: f32, to_velocity_y_a: f32, entity_b: Entity, type_b: String, to_velocity_x_b: f32, to_velocity_y_b: f32) -> CollisionEvent {
        CollisionEvent {entity_a, type_a, to_velocity_x_a, to_velocity_y_a, entity_b, type_b, to_velocity_x_b, to_velocity_y_b}
    }
}

pub struct SpaceShooter {
    dispatcher: Dispatcher<'static, 'static>,
}

impl Default for SpaceShooter {
    fn default() -> Self {
        SpaceShooter {
            dispatcher: DispatcherBuilder::new()
                .with(systems::PlanetsSystem, "planets_system", &[])
                .with(systems::GameMasterSystem, "gamemaster_system", &[])
                .with(systems::SpaceshipSystem, "spaceship_system", &[])
                .with(systems::EnemySystem, "enemy_system", &[])
                .with(systems::ConsumableSystem, "consumable_system", &[])
                .with(systems::SpawnerSystem, "spawner_system", &[])
                .with(systems::PlayerHitSystem, "player_hit_system", &[])
                .with(systems::EnemyHitSystem, "enemy_hit_system", &[])
                .with(systems::ExplosionSystem, "explosion_system", &[])
                .with(systems::ItemSystem, "item_system", &[])
                .with(systems::SpaceshipMovementSystem, "spaceship_movement_system", &[])
                //.with(systems::ItemSpawnSystem, "item_spawn_system", &[])
                .with(systems::StatusBarSystem, "status_bar_system", &[])
                .with(systems::CollisionDetectionSystem, "collision_detection_system", &[])
                .with(systems::CollisionHandlerSystem::default(), "collision_handler_system", &["collision_detection_system"])
                .with(systems::DefenseSystem, "defense_system", &[])
                .with(systems::BlastSystem, "blast_system", &[])
                .with(systems::StoreSystem, "store_system", &[])
                .with(systems::StatTrackerSystem, "stat_tracker_system", &["store_system", "spaceship_system", "consumable_system"])
                .build(),
        }
    }
}

impl SimpleState for SpaceShooter {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

        let world = data.world;
        //let background_sprite_sheet_handle = load_spritesheet(world, "earth_planet_background.png", "earth_planet_background.ron");
        let side_panel_sprite_sheet_handle = load_spritesheet(world, "side_panel_spritesheet.png", "side_panel_spritesheet.ron");
        let items_sprite_sheet_handle = load_spritesheet(world, "items_spritesheet.png", "items_spritesheet.ron");
        let consumables_sprite_sheet_handle = load_spritesheet(world, "consumables_spritesheet.png", "consumables_spritesheet.ron");
        let status_bar_unit_sprite_sheet_handle = load_spritesheet(world, "status_bar_unit_spritesheet.png", "status_bar_unit_spritesheet.ron");
        let enemies_sprite_sheet_handle = load_spritesheet(world, "enemies_spritesheet.png", "enemies_spritesheet.ron");
        let players_sprite_sheet_handle = load_spritesheet(world, "player_spritesheet.png", "player_spritesheet.ron");
        let blasts_sprite_sheet_handle = load_spritesheet(world, "blasts_spritesheet.png", "blasts_spritesheet.ron");
        let explosions_sprite_sheet_handle = load_spritesheet(world, "explosions_spritesheet.png", "explosions_spritesheet.ron");

        self.dispatcher.setup(world);

        initialise_audio(world);
        initialise_ui(world);
        initialise_gamemaster(world);
        initialise_defense(world);
        initialise_status_bars(world);
        initialise_planet(world, "earth_planet.glb", ARENA_MIN_X + (ARENA_WIDTH/2.0), -1100.0, -1010.0, 1000.0, 100.0, 0.01);
        initialise_planet(world, "sol_star.glb", ARENA_MIN_X + (ARENA_WIDTH/2.0) - 5000.0, (ARENA_HEIGHT/2.0) + 3000.0, -15000.0, 800.0, 0.0, 0.005);
        //initialise_background(world, background_sprite_sheet_handle);
        initialise_spaceship(world, players_sprite_sheet_handle.clone());
        initialise_sprite_resource(world,
                                   items_sprite_sheet_handle,
                                   consumables_sprite_sheet_handle,
                                   status_bar_unit_sprite_sheet_handle,
                                   enemies_sprite_sheet_handle,
                                   players_sprite_sheet_handle,
                                   blasts_sprite_sheet_handle,
                                   explosions_sprite_sheet_handle);
        initialise_enemy_spawner(world);
        initialise_side_panels(world, side_panel_sprite_sheet_handle);
        initialise_store(world);
        initialise_camera(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        self.dispatcher.dispatch(data.world);
        Trans::None
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {

        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Push(Box::new(PausedState));
            }
        }
        Trans::None
    }
}

pub struct PausedState;

impl SimpleState for PausedState {

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans  {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Pop;
            }
        }
        Trans::None
    }
}

fn load_spritesheet(world: &mut World, spritesheet: &str, spritesheet_ron: &str) -> Handle<SpriteSheet> {

    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("texture/{}", spritesheet),
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("texture/{}", spritesheet_ron),
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(GAME_WIDTH * 0.5, GAME_HEIGHT * 0.5, 237.0);
    //transform.set_translation_xyz(GAME_WIDTH * 0.5, GAME_HEIGHT * 0.5, 400.0);
    //transform.set_translation_xyz(0.0, 0.0, 500.0);
    //transform.set_translation_xyz(0.0, 0.0, 300.0);
    //transform.set_rotation_euler(0.0, 15.0_f32.to_radians(), 0.0);
    transform.set_rotation_euler(0.0, 0.0, 0.0);

    world
        .create_entity()
        .with(Camera::from(camera::Projection::perspective(
           1.3,
            FRAC_PI_3,
            0.1,
            20000.0
        )))
        .with(transform)
        .build();
}

pub struct TrackedStats {
    pub currency: Entity,
    pub item_price_1: Entity,
    pub item_price_2: Entity,
    pub item_price_3: Entity,
}

fn initialise_ui(world:  &mut World) {

    let item_slots_texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/item_slots.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let item_slots_sprite_sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "texture/item_slots.ron",
            SpriteSheetFormat(item_slots_texture_handle),
            (),
            &sprite_sheet_store,
        )
    };

    let item_slots_sprite_render = SpriteRender {
        sprite_sheet: item_slots_sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_MAX_X + 10.0 + 2.0, ARENA_MIN_Y + 29.0 + 24.0, 0.9);

    world.create_entity()
        .with(item_slots_sprite_render)
        .with(local_transform)
        .build();

    let currency_texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/currency_ui.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let currency_sprite_sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "texture/currency_ui.ron",
            SpriteSheetFormat(currency_texture_handle),
            (),
            &sprite_sheet_store,
        )
    };

    let currency_sprite_render = SpriteRender {
        sprite_sheet: currency_sprite_sheet_handle.clone(),
        sprite_number: 0,
    };
    
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_MAX_X + 10.0, ARENA_MIN_Y + 12.5, 0.9);

    world.create_entity()
        .with(currency_sprite_render)
        .with(local_transform)
        .build();

    let font = world.read_resource::<Loader>().load(
        "font/SpaceMadness.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let currency_count_transform = UiTransform::new("currency_count".to_string(), Anchor::BottomRight, Anchor::BottomRight, -6.0, 10.0, 0.9, 50.0, 45.0);
    let currency_count = world
        .create_entity()
        .with(currency_count_transform)
        .with(UiText::new(
            font.clone(),
            "x0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            20.0
        )).build();

    let item_price_1_transform = UiTransform::new("item_price_0".to_string(), Anchor::BottomRight, Anchor::BottomRight, -6.0, 130.0, 0.9, 50.0, 45.0);
    let item_price_1 = world
        .create_entity()
        .with(item_price_1_transform)
        .with(UiText::new(
            font.clone(),
            "$0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            15.0
        )).build();

    let item_price_2_transform = UiTransform::new("item_price_1".to_string(), Anchor::BottomRight, Anchor::BottomRight, -6.0, 90.0, 0.9, 50.0, 45.0);
    let item_price_2 = world
        .create_entity()
        .with(item_price_2_transform)
        .with(UiText::new(
            font.clone(),
            "$0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            15.0
        )).build();

    let item_price_3_transform = UiTransform::new("item_price_2".to_string(), Anchor::BottomRight, Anchor::BottomRight, -6.0, 50.0, 0.9, 50.0, 45.0);
    let item_price_3 = world
        .create_entity()
        .with(item_price_3_transform)
        .with(UiText::new(
            font.clone(),
            "$0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            15.0
        )).build();

    world.add_resource(TrackedStats {
        currency: currency_count,
        item_price_1: item_price_1,
        item_price_2: item_price_2,
        item_price_3: item_price_3,
    });
    //world.add_resource(currency_icon);
}
