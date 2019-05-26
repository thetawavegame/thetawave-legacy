use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::{Transform},
    prelude::*,
    renderer::{
        Camera, PngFormat, Projection, SpriteSheet,
        SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
    },
};

use crate::entities::{initialise_sprite_resource, initialise_spaceship, initialise_spawner};
//use crate::components::Enemy;


pub const GAME_HEIGHT: f32 = 250.0;
pub const GAME_WIDTH: f32 = 250.0;


pub struct SpaceShooter;
impl SimpleState for SpaceShooter {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_spritesheet(world, "spritesheet.png", "spritesheet.ron");

        //world.register::<Spaceship>();
        //world.register::<Blast>();
        initialise_spaceship(world, sprite_sheet_handle.clone());
        initialise_sprite_resource(world, sprite_sheet_handle);
        initialise_spawner(world);
        initialise_camera(world);
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
