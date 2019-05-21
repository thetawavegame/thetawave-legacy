use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::{Transform, TransformBundle},
    core::nalgebra::Vector3,
    ecs::prelude::{Component, DenseVecStorage, Entity, Entities, LazyUpdate, ReadExpect},
    prelude::*,
    renderer::{
        Camera, PngFormat, Projection, SpriteRender, SpriteSheet,
        SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
        Flipped,
    },
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

pub const GAME_HEIGHT: f32 = 250.0;
pub const GAME_WIDTH: f32 = 250.0;

pub const SPACESHIP_HEIGHT: f32 = 18.0;
pub const SPACESHIP_WIDTH: f32 = 18.0;
const SPACESHIP_STARTING_SPEED: f32 = 70.0;
const SPACESHIP_STARTING_FIRE_SPEED: f32 = 0.5;
const BLAST_STARTING_SPEED: f32 = 100.0;

pub struct SpaceShooter;
impl SimpleState for SpaceShooter {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_spritesheet(world);

        //world.register::<Spaceship>();
        //world.register::<Blast>();
        initialise_spaceship(world, sprite_sheet_handle.clone());
        initialise_blast_resource(world, sprite_sheet_handle);
        initialise_camera(world);
    }

}

#[derive(Clone)]
pub struct Blast {
    pub speed: f32,
}

impl Component for Blast {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone)]
pub struct BlastResource {
    pub sprite_sheet: Handle<SpriteSheet>,
    pub sprite_number: usize,
}

pub fn initialise_blast_resource(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) -> BlastResource {
    let blast_resource = BlastResource {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1,
    };

    world.add_resource(blast_resource.clone());
    blast_resource
}

pub fn fire_blast(entities: &Entities, blast_resource: &ReadExpect<BlastResource>, fire_position: Vector3<f32>, lazy_update: &ReadExpect<LazyUpdate>) {
    let blast_entity: Entity = entities.create();

    let mut local_transform = Transform::default();
    local_transform.set_position(fire_position);

    let sprite_render = SpriteRender {
        sprite_sheet: blast_resource.sprite_sheet.clone(),
        sprite_number: blast_resource.sprite_number,
    };

    lazy_update.insert(blast_entity, sprite_render);
    lazy_update.insert(blast_entity, Blast {speed: BLAST_STARTING_SPEED});
    lazy_update.insert(blast_entity, local_transform);

}

pub struct Spaceship {
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub fire_speed: f32,
    pub fire_reset_timer: f32
}

impl Component for Spaceship {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_spaceship(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {

    //create transform
    let mut local_transform = Transform::default();
    local_transform.set_xyz(GAME_WIDTH / 2.0, GAME_HEIGHT / 6.0, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Spaceship {
            width: SPACESHIP_WIDTH,
            height: SPACESHIP_HEIGHT,
            speed: SPACESHIP_STARTING_SPEED,
            fire_speed: SPACESHIP_STARTING_FIRE_SPEED,
            fire_reset_timer: 0.0,
        })
        .with(local_transform)
        .build();
}

fn load_spritesheet(world: &mut World) -> SpriteSheetHandle {

    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/space_shooter_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/space_shooter_spritesheet.ron",
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
