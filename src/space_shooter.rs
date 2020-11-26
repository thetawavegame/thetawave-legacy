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
    resources::initialize_sprite_resource,
    systems,
};
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::prelude::{Dispatcher, DispatcherBuilder, Entity},
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::formats::texture::ImageFormat,
    renderer::{Camera, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform},
};
use std::f32::consts::FRAC_PI_3;

pub struct SpaceShooter {
    is_paused: bool,
    pause_display: Option<Entity>,
    dispatcher: Dispatcher<'static, 'static>,
}

impl Default for SpaceShooter {
    fn default() -> Self {
        SpaceShooter {
            is_paused: false,
            pause_display: None,
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

impl SimpleState for SpaceShooter {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        //let background_sprite_sheet_handle = load_spritesheet(world, "earth_planet_background.png", "earth_planet_background.ron");
        let side_panel_sprite_sheet_handle = load_spritesheet(
            world,
            "side_panel_spritesheet.png",
            "side_panel_spritesheet.ron",
        );
        let items_sprite_sheet_handle =
            load_spritesheet(world, "items_spritesheet.png", "items_spritesheet.ron");
        let consumables_sprite_sheet_handle = load_spritesheet(
            world,
            "consumables_spritesheet.png",
            "consumables_spritesheet.ron",
        );
        let status_bar_unit_sprite_sheet_handle = load_spritesheet(
            world,
            "status_bar_unit_spritesheet.png",
            "status_bar_unit_spritesheet.ron",
        );
        let enemies_sprite_sheet_handle =
            load_spritesheet(world, "enemies_spritesheet.png", "enemies_spritesheet.ron");
        let players_sprite_sheet_handle =
            load_spritesheet(world, "player_spritesheet.png", "player_spritesheet.ron");
        let blasts_sprite_sheet_handle =
            load_spritesheet(world, "blasts_spritesheet.png", "blasts_spritesheet.ron");
        let explosions_sprite_sheet_handle = load_spritesheet(
            world,
            "explosions_spritesheet.png",
            "explosions_spritesheet.ron",
        );
        let repeater_sprite_sheet_handle = load_spritesheet(
            world,
            "repeater_spritesheet.png",
            "repeater_spritesheet.ron",
        );
        let blast_explosions_sprite_sheet_handle =
            load_spritesheet(world, "blast_explosions.png", "blast_explosions.ron");
        let thrusters_sprite_sheet_handle = load_spritesheet(
            world,
            "thrusters_spritesheet.png",
            "thrusters_spritesheet.ron",
        );

        self.dispatcher.setup(world);

        initialize_audio(world);
        initialise_ui(world);
        initialize_gamemaster(world);
        initialize_defense(world);
        initialize_status_bars(world);
        initialize_planet(
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
            world,
            "sol_star.glb",
            ARENA_MIN_X + (ARENA_WIDTH / 2.0) - 5000.0,
            (ARENA_HEIGHT / 2.0) + 3000.0,
            -15000.0,
            800.0,
            0.0,
            0.005,
        );
        initialize_spaceship(world, players_sprite_sheet_handle.clone());
        initialize_sprite_resource(
            world,
            items_sprite_sheet_handle,
            consumables_sprite_sheet_handle,
            status_bar_unit_sprite_sheet_handle,
            enemies_sprite_sheet_handle,
            players_sprite_sheet_handle,
            blasts_sprite_sheet_handle,
            explosions_sprite_sheet_handle,
            repeater_sprite_sheet_handle,
            blast_explosions_sprite_sheet_handle,
            thrusters_sprite_sheet_handle,
        );
        initialize_enemy_spawner(world);
        initialize_side_panels(world, side_panel_sprite_sheet_handle);
        initialize_store(world);
        initialise_camera(world);
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

pub struct PausedState;

impl SimpleState for PausedState {
    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Pop;
            }
        }
        Trans::None
    }
}

fn load_spritesheet(
    world: &mut World,
    spritesheet: &str,
    spritesheet_ron: &str,
) -> Handle<SpriteSheet> {
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
    transform.set_translation_xyz(CAMERA_X, CAMERA_Y, CAMERA_Z);
    transform.set_rotation_euler(0.0, 0.0, 0.0);

    world
        .create_entity()
        .with(Camera::perspective(1.3, FRAC_PI_3, 0.1))
        .with(transform)
        .build();
}

pub struct TrackedStats {
    pub currency: Entity,
    pub item_price_1: Entity,
    pub item_price_2: Entity,
    pub item_price_3: Entity,
}

fn initialise_ui(world: &mut World) {
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
