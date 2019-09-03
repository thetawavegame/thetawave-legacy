use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::{Transform},
    prelude::*,
    renderer::{
        Camera, SpriteSheet,
        SpriteSheetFormat, Texture,
        SpriteRender,
    },
    input::{
        is_key_down,
        VirtualKeyCode,
    },
    ecs::prelude::{Dispatcher, DispatcherBuilder, Entity},
    renderer::{
        formats::texture::ImageFormat,
    },
    ui::{
        TtfFormat,
        Anchor,
        UiText,
        UiTransform,
    },
};

use crate::audio::initialise_audio;

use crate::systems;
use crate::entities::{initialise_gamemaster, initialise_sprite_resource, initialise_spaceship, initialise_enemy_spawner, initialise_item_spawner, initialise_side_panels, initialise_background, initialise_defense, initialise_status_bars};

//GAME_HEIGHT and _WIDTH should be  half the resolution?
pub const GAME_WIDTH: f32 = 360.0;
pub const GAME_HEIGHT: f32 = 270.0;
pub const ARENA_MIN_Y: f32 = 0.0;
pub const ARENA_MAX_Y: f32 = GAME_HEIGHT - ARENA_MIN_Y;
pub const ARENA_MIN_X: f32 = GAME_WIDTH / 8.0;
pub const ARENA_MAX_X: f32 = GAME_WIDTH - ARENA_MIN_X;
pub const ARENA_HEIGHT: f32 = ARENA_MAX_Y - ARENA_MIN_Y;
pub const ARENA_WIDTH: f32 = ARENA_MAX_X - ARENA_MIN_X;
pub const ARENA_SPAWN_OFFSET: f32 = 20.0;

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
                .with(systems::ItemSpawnSystem, "item_spawn_system", &[])
                .with(systems::StatusBarSystem, "status_bar_system", &[])
                .with(systems::CollisionDetectionSystem, "collision_detection_system", &[])
                .with(systems::CollisionHandlerSystem::default(), "collision_handler_system", &[])
                .with(systems::DefenseSystem, "defense_system", &[])
                .with(systems::BlastSystem, "blast_system", &[])
                .with(systems::StatTrackerSystem, "stat_tracker_system", &[])
                .build(),
        }
    }
}

impl SimpleState for SpaceShooter {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {


        let world = data.world;
        let sprite_sheet_handle = load_spritesheet(world, "spritesheet.png", "spritesheet.ron");
        let background_sprite_sheet_handle = load_spritesheet(world, "earth_planet_background.png", "earth_planet_background.ron");
        let side_panel_sprite_sheet_handle = load_spritesheet(world, "side_panel_spritesheet.png", "side_panel_spritesheet.ron");


        self.dispatcher.setup(&mut world.res);
        initialise_audio(world);
        initialise_ui(world);
        initialise_gamemaster(world);
        initialise_defense(world);
        initialise_status_bars(world);
        initialise_background(world, background_sprite_sheet_handle);
        initialise_spaceship(world, sprite_sheet_handle.clone());
        initialise_sprite_resource(world, sprite_sheet_handle);
        initialise_enemy_spawner(world);
        initialise_item_spawner(world);
        initialise_side_panels(world, side_panel_sprite_sheet_handle);
        initialise_camera(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        self.dispatcher.dispatch(&mut data.world.res);
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
    transform.set_translation_xyz(GAME_WIDTH * 0.5, GAME_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(GAME_WIDTH, GAME_HEIGHT))
        .with(transform)
        .build();
}

pub struct TrackedStats {
    pub currency: Entity,
}

fn initialise_ui(world:  &mut World) {

    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/currency_ui.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let sprite_sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "texture/currency_ui.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sprite_sheet_store,
        )
    };

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };
    
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_MAX_X + 10.0, ARENA_MIN_Y + 12.5, 0.9);

    world.create_entity()
        .with(sprite_render)
        .with(local_transform)
        .build();

    let font = world.read_resource::<Loader>().load(
        "font/Teko-SemiBold.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let currency_count_transform = UiTransform::new("currency_count".to_string(), Anchor::BottomRight, Anchor::BottomRight, -10.0, 10.0, 0.9, 50.0, 45.0);
    let currency_count = world
        .create_entity()
        .with(currency_count_transform)
        .with(UiText::new(
            font.clone(),
            "x 0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            45.0
        )).build();

    world.add_resource(TrackedStats {
        currency: currency_count
    });
    //world.add_resource(currency_icon);
}
