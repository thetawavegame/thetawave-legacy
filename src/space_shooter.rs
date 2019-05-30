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
use crate::entities::{initialise_sprite_resource, initialise_spaceship, initialise_enemy_spawner, initialise_item_spawner};
use crate::components::ItemSpawner;


pub const GAME_HEIGHT: f32 = 250.0;
pub const GAME_WIDTH: f32 = 250.0;


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
                .with(systems::SpawnerSystem, "spawner_system", &[])
                .with(systems::PlayerHitSystem, "player_hit_system", &[])
                .with(systems::ExplosionSystem, "explosion_system", &[])
                .with(systems::ItemSystem, "item_system", &[])
                .with(systems::BarrelRollSystem, "barrel_roll_system", &[])
                .with(systems::SpaceshipMovementSystem, "spaceship_movement_system", &[])
                .with(systems::ItemSpawnSystem, "item_spawn_system", &[])
                .build(),
        }
    }
}

impl SimpleState for SpaceShooter {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {


        let world = data.world;
        let sprite_sheet_handle = load_spritesheet(world, "spritesheet.png", "spritesheet.ron");

        self.dispatcher.setup(&mut world.res);

        //world.register::<Spaceship>();
        //world.register::<Blast>();
        world.register::<ItemSpawner>();
        println!("in spaceshooter state");
        initialise_spaceship(world, sprite_sheet_handle.clone());
        initialise_sprite_resource(world, sprite_sheet_handle);
        initialise_enemy_spawner(world);
        initialise_item_spawner(world);
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

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("paused state");
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
       println!("exit paused state");
    }

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
