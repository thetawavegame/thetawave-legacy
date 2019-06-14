use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::{Transform},
    prelude::*,
    renderer::{
        Camera, PngFormat, Projection, SpriteSheet,
        SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata, VirtualKeyCode,
    },
    input::is_key_down,
    ecs::prelude::{Dispatcher, DispatcherBuilder}
};

use crate::systems;
use crate::entities::{initialise_sprite_resource, initialise_spaceship, initialise_enemy_spawner, initialise_item_spawner, initialise_side_panels, initialise_background, initialise_defense, initialise_status_bars};

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


pub struct SpaceShooter {
    dispatcher: Dispatcher<'static, 'static>,
}

impl Default for SpaceShooter {
    fn default() -> Self {
        SpaceShooter {
            dispatcher: DispatcherBuilder::new()
                .with(systems::SpaceshipSystem, "spaceship_system", &[])
                .with(systems::BlastSystem, "blast_system", &[])
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
                .with(systems::SpaceshipEnemyCollisionSystem, "spaceship_enemy_collision_system", &[])
                .with(systems::EnemyEnemyCollisionSystem, "enemy_enemy_collision_system", &[])
                .with(systems::DefenseSystem, "defense_system", &[])
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

fn load_spritesheet(world: &mut World, spritesheet: &str, spritesheet_ron: &str) -> SpriteSheetHandle {

    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("texture/{}", spritesheet),
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("texture/{}", spritesheet_ron),
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);

    world.create_entity().with(Camera::from(Projection::orthographic(
        0.0,
        GAME_WIDTH,
        0.0,
        GAME_HEIGHT,
    ))).with(transform).build();
}
