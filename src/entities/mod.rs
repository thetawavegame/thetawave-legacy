use crate::resources::SpriteResource;
use amethyst::ecs::prelude::World;
use amethyst::renderer::SpriteSheetHandle;

pub mod blast;
pub mod spaceship;
pub mod enemy;
pub mod enemy_spawner;
pub mod explosion;
pub mod items;
pub mod item_spawner;
pub mod side_panel;
pub mod background;
pub mod consumable;
pub mod defense;
pub mod status_bar;
pub mod status_unit;

pub use self::{
    blast::{fire_blast},
    spaceship::initialise_spaceship,
    enemy::{spawn_enemy},
    enemy_spawner::{initialise_enemy_spawner},
    explosion::{spawn_explosion},
    item_spawner::initialise_item_spawner,
    items::spawn_item,
    side_panel::initialise_side_panels,
    background::initialise_background,
    consumable::spawn_consumable,
    defense::initialise_defense,
    status_bar::initialise_status_bars,
    status_unit::spawn_status_unit,
};

pub fn initialise_sprite_resource(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) -> SpriteResource {
    let sprite_resource = SpriteResource {
        sprite_sheet: sprite_sheet_handle,
    };

    world.add_resource(sprite_resource.clone());
    sprite_resource
}