use crate::{
    audio::initialize_audio,
    constants::{
        ARENA_HEIGHT, ARENA_MAX_X, ARENA_MIN_X, ARENA_MIN_Y, ARENA_WIDTH, CAMERA_X, CAMERA_Y,
        CAMERA_Z,
    },
    entities::{
        initialize_defense, initialize_enemy_spawner, initialize_gamemaster, initialize_planet,
        initialize_side_panels, initialize_spaceship, initialize_status_bars, initialize_store,
    },
    resources::{DebugLinesConfig, SpriteSheetsConfig, SpriteSheetsResource},
    states::PausedState,
    systems,
};
use amethyst::{
    assets::{AssetStorage, Loader, ProgressCounter},
    core::transform::Transform,
    ecs::prelude::{Dispatcher, DispatcherBuilder, Entity},
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::debug_drawing::{DebugLines, DebugLinesParams},
    renderer::formats::texture::ImageFormat,
    renderer::{Camera, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform},
};
use std::{collections::HashMap, f32::consts::FRAC_PI_3};

pub struct MainGameState {
    is_paused: bool,
    pause_display: Option<Entity>,
    dispatcher: Dispatcher<'static, 'static>,
    progress_counter: ProgressCounter,
    loading_display: Option<Entity>,
    spritesheets: Option<SpriteSheetsResource>,
    is_loaded: bool,
}

impl Default for MainGameState {
    fn default() -> Self {
        MainGameState {
            is_paused: false,
            pause_display: None,
            progress_counter: ProgressCounter::new(),
            loading_display: None,
            spritesheets: None,
            is_loaded: false,
            dispatcher: DispatcherBuilder::new()
                .with(systems::AnimationSystem, "animation_system", &[])
                .with(systems::PlanetsSystem, "planets_system", &[])
                .with(systems::GameMasterSystem, "gamemaster_system", &[])
                .with(systems::EnemySystem, "enemy_system", &[])
                .with(systems::BossSystem, "boss_system", &[])
                .with(systems::ConsumableSystem, "consumable_system", &[])
                .with(systems::SpawnerSystem, "spawner_system", &[])
                .with(systems::ItemSystem, "item_system", &[])
                .with(systems::TimeLimitSystem, "timelimit_system", &[])
                .with(
                    systems::SpaceshipMovementSystem,
                    "spaceship_movement_system",
                    &[],
                )
                .with(systems::StatusBarSystem, "status_bar_system", &[])
                .with(
                    systems::CollisionDetectionSystem,
                    "collision_detection_system",
                    &[],
                )
                .with(
                    systems::CollisionHandlerSystem::default(),
                    "collision_handler_system",
                    &["collision_detection_system"],
                )
                .with(
                    systems::SpaceshipEnemyCollisionSystem::default(),
                    "spaceship_enemy_collision_system",
                    &["collision_handler_system"],
                )
                .with(
                    systems::SpaceshipBlastCollisionSystem::default(),
                    "spaceship_blast_collision_system",
                    &["collision_handler_system"],
                )
                .with(
                    systems::SpaceshipItemCollisionSystem::default(),
                    "spaceship_item_collision_system",
                    &["collision_handler_system"],
                )
                .with(
                    systems::SpaceshipConsumableCollisionSystem::default(),
                    "spaceship_consumable_collision_system",
                    &["collision_handler_system"],
                )
                .with(
                    systems::EnemyPlayerCollisionSystem::default(),
                    "enemy_player_collision",
                    &["collision_handler_system"],
                )
                .with(
                    systems::EnemyEnemyCollisionSystem::default(),
                    "enemy_enemy_collision",
                    &["collision_handler_system"],
                )
                .with(
                    systems::EnemyBlastCollisionSystem::default(),
                    "enemy_blast_collision",
                    &["collision_handler_system"],
                )
                .with(
                    systems::DefenseSystem::default(),
                    "defense_system",
                    &["spaceship_item_collision_system"],
                )
                .with(
                    systems::SpaceshipSystem::default(),
                    "spaceship_system",
                    &["spaceship_item_collision_system"],
                )
                .with(systems::BlastSystem, "blast_system", &[])
                .with(systems::StoreSystem, "store_system", &[])
                .with(
                    systems::StatTrackerSystem,
                    "stat_tracker_system",
                    &["store_system", "spaceship_system"],
                )
                .with(systems::AutoFireSystem, "autoblaster_system", &[])
                .with(systems::ManualBlasterSystem, "manualblaster_system", &[])
                .with(
                    systems::EnemyDestroyedSystem::default(),
                    "enemy_destroyed_system",
                    &["enemy_system"],
                )
                .with(
                    systems::PlayAudioSystem::default(),
                    "play_audio_system",
                    &[],
                )
                .build(),
        }
    }
}

impl SimpleState for MainGameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.dispatcher.setup(world);
        let spritesheets = init_spritesheets(&mut self.progress_counter, world);
        self.spritesheets = Some(spritesheets);

        initialize_audio(&mut self.progress_counter, world);
        initialise_ui(&mut self.progress_counter, world);
        initialize_planet(
            &mut self.progress_counter,
            world,
            "earth_planet.glb",
            ARENA_MIN_X + (ARENA_WIDTH / 2.0),
            -1100.0,
            -1010.0,
            1000.0,
            100.0,
            0.01,
        );
        initialize_planet(
            &mut self.progress_counter,
            world,
            "sol_star.glb",
            ARENA_MIN_X + (ARENA_WIDTH / 2.0) - 5000.0,
            (ARENA_HEIGHT / 2.0) + 3000.0,
            -15000.0,
            800.0,
            0.0,
            0.005,
        );

        self.loading_display = Some(initialise_loading_text(self.progress_counter.num_assets(), world));

        world.insert(DebugLines::new());
        let debug_lines_params = {
            let debug_lines_config = world.read_resource::<DebugLinesConfig>();

            DebugLinesParams {
                line_width: debug_lines_config.line_width,
            }
        };
        world.insert(debug_lines_params);
    }

    fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.is_paused = true;

        let paused_text_entity = get_paused_text(data.world);
        self.pause_display = Some(paused_text_entity);
    }

    fn on_resume(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        self.is_paused = false;
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        self.dispatcher.dispatch(data.world);

        // Handle paused state here.
        if let Some(pause_text_entity) = self.pause_display {
            data.world
                .delete_entity(pause_text_entity)
                .expect("Failed to remove Paused text.");

            self.pause_display = None;
        }

        // Handle loading state here
        if self.progress_counter.is_complete() && !self.is_loaded {
            self.is_loaded = true;

            if let Some(loading_display_entity) = self.loading_display {
                data.world
                    .delete_entity(loading_display_entity)
                    .expect("Failed to remove loading text.");
            }

            initialize_spaceship(data.world, self.spritesheets.as_ref().unwrap().spritesheets["characters"].clone());
            initialize_enemy_spawner(data.world);

            initialize_gamemaster(data.world);
            initialize_defense(data.world);
            initialize_status_bars(data.world);
            initialize_side_panels(data.world, self.spritesheets.as_ref().unwrap().spritesheets["side_panels"].clone());
            initialize_store(data.world);
            initialise_camera(data.world);

        } else if !self.is_loaded {
            let mut ui_text = data.world.write_storage::<UiText>();

            if let Some(loading_display) = self.loading_display.and_then(|entity| ui_text.get_mut(entity)) {
                loading_display.text = format!("Loading {}/{} assets", self.progress_counter.num_finished(), self.progress_counter.num_assets());
            }
        }

        Trans::None
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Push(Box::new(PausedState));
            }
        }
        Trans::None
    }
}

fn init_spritesheets(progress_counter: &mut ProgressCounter, world: &mut World) -> SpriteSheetsResource {
    let mut spritesheets = HashMap::new();
    {
        let spritesheets_config = world.read_resource::<SpriteSheetsConfig>();
        for (spritesheet_name, spritesheet_data) in spritesheets_config.iter() {
            let texture_handle = {
                let loader = world.read_resource::<Loader>();
                let texture_storage = world.read_resource::<AssetStorage<Texture>>();
                loader.load(
                    format!("texture/{}", spritesheet_data.image),
                    ImageFormat::default(),
                    &mut *progress_counter,
                    &texture_storage,
                )
            };

            let loader = world.read_resource::<Loader>();
            let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
            let spritesheet_handle = loader.load(
                format!("texture/{}", spritesheet_data.data),
                SpriteSheetFormat(texture_handle),
                &mut *progress_counter,
                &sprite_sheet_store,
            );

            spritesheets.insert(spritesheet_name.clone(), spritesheet_handle);
        }
    }
    let spritesheets = SpriteSheetsResource { spritesheets };
    world.insert(spritesheets.clone());
    spritesheets
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(CAMERA_X, CAMERA_Y, CAMERA_Z);

    world
        .create_entity()
        .with(Camera::perspective(1.3, FRAC_PI_3, 0.1))
        .with(transform)
        .build();
}

pub struct TrackedStats {
    pub currency: Entity,
    pub shields: Entity,
    pub item_price_1: Entity,
    pub item_price_2: Entity,
    pub item_price_3: Entity,
}

fn initialise_ui(progress_counter: &mut ProgressCounter, world: &mut World) {
    let item_slots_texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/item_slots.png",
            ImageFormat::default(),
            &mut *progress_counter,
            &texture_storage,
        )
    };

    let item_slots_sprite_sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "texture/item_slots.ron",
            SpriteSheetFormat(item_slots_texture_handle),
            &mut *progress_counter,
            &sprite_sheet_store,
        )
    };

    let item_slots_sprite_render = SpriteRender {
        sprite_sheet: item_slots_sprite_sheet_handle,
        sprite_number: 0,
    };

    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_MAX_X + 10.0 + 2.0, ARENA_MIN_Y + 29.0 + 24.0, 0.9);

    world
        .create_entity()
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

    let consumables_texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/consumables_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let consumables_sprite_sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "texture/consumables_spritesheet.ron",
            SpriteSheetFormat(consumables_texture_handle),
            (),
            &sprite_sheet_store,
        )
    };

    let currency_sprite_render = SpriteRender {
        sprite_sheet: currency_sprite_sheet_handle,
        sprite_number: 0,
    };

    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_MAX_X + 10.0, ARENA_MIN_Y + 12.5, 0.9);

    world
        .create_entity()
        .with(currency_sprite_render)
        .with(local_transform)
        .build();

    let shield_sprite_render = SpriteRender {
        sprite_sheet: consumables_sprite_sheet_handle,
        sprite_number: 4,
    };

    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_MAX_X + 10.0, ARENA_MIN_Y + 158.0, 0.9);

    world
        .create_entity()
        .with(shield_sprite_render)
        .with(local_transform)
        .build();

    let font = world.read_resource::<Loader>().load(
        "font/SpaceMadness.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let currency_count_transform = UiTransform::new(
        "currency_count".to_string(),
        Anchor::BottomRight,
        Anchor::BottomRight,
        -6.0,
        10.0,
        0.9,
        50.0,
        45.0,
    );
    let currency_count = world
        .create_entity()
        .with(currency_count_transform)
        .with(UiText::new(
            font.clone(),
            "x0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            20.0,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    let shields_count_transform = UiTransform::new(
        "shields_count".to_string(),
        Anchor::MiddleRight,
        Anchor::MiddleRight,
        -6.0,
        48.0,
        0.9,
        50.0,
        45.0,
    );

    let shields_count = world
        .create_entity()
        .with(shields_count_transform)
        .with(UiText::new(
            font.clone(),
            "x0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            20.0,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    let item_price_1_transform = UiTransform::new(
        "item_price_0".to_string(),
        Anchor::BottomRight,
        Anchor::BottomRight,
        -6.0,
        130.0,
        0.9,
        50.0,
        45.0,
    );
    let item_price_1 = world
        .create_entity()
        .with(item_price_1_transform)
        .with(UiText::new(
            font.clone(),
            "$0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            15.0,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    let item_price_2_transform = UiTransform::new(
        "item_price_1".to_string(),
        Anchor::BottomRight,
        Anchor::BottomRight,
        -6.0,
        90.0,
        0.9,
        50.0,
        45.0,
    );
    let item_price_2 = world
        .create_entity()
        .with(item_price_2_transform)
        .with(UiText::new(
            font.clone(),
            "$0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            15.0,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    let item_price_3_transform = UiTransform::new(
        "item_price_2".to_string(),
        Anchor::BottomRight,
        Anchor::BottomRight,
        -6.0,
        50.0,
        0.9,
        50.0,
        45.0,
    );
    let item_price_3 = world
        .create_entity()
        .with(item_price_3_transform)
        .with(UiText::new(
            font,
            "$0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            15.0,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    world.insert(TrackedStats {
        currency: currency_count,
        shields: shields_count,
        item_price_1,
        item_price_2,
        item_price_3,
    });
}

fn get_paused_text(world: &mut World) -> Entity {
    let font_handle = world.read_resource::<Loader>().load(
        "font/SpaceMadness.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let ui_transform = UiTransform::new(
        String::from("paused_text"),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        0.0,
        0.0,
        100.0,
        30.0,
    );
    let ui_text = UiText::new(
        font_handle,
        String::from("paused"),
        [1.0, 1.0, 1.0, 1.0],
        25.0,
        LineMode::Single,
        Anchor::Middle,
    );

    let entity = world
        .create_entity()
        .with(ui_transform)
        .with(ui_text)
        .build();

    entity
}

fn initialise_loading_text(num_assets: usize, world: &mut World) -> Entity {
    let font_handle = world.read_resource::<Loader>().load(
        "font/SpaceMadness.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let ui_transform = UiTransform::new(
        String::from("progress_text"),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        0.0,
        0.0,
        900.0,
        500.0,
    );
    let text = "0/".to_string() + &num_assets.to_string();
    let ui_text = UiText::new(
        font_handle,
        String::from(text),
        [1.0, 1.0, 1.0, 1.0],
        64.0,
        LineMode::Single,
        Anchor::Middle,
    );

    let entity = world
        .create_entity()
        .with(ui_transform)
        .with(ui_text)
        .build();

    entity
}