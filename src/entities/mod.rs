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
pub mod health_bar;
pub mod health_unit;
pub mod defense_bar;
pub mod defense_unit;
pub mod roll_bar;
pub mod roll_unit;
pub mod consumable;
pub mod defense;

pub use self::{
    blast::{fire_blast},
    spaceship::initialise_spaceship,
    enemy::{spawn_enemy},
    enemy_spawner::{initialise_enemy_spawner, ENEMY_WIDTH},
    explosion::{spawn_explosion},
    item_spawner::initialise_item_spawner,
    items::spawn_item,
    side_panel::initialise_side_panels,
    background::initialise_background,
    health_bar::initialise_health_bar,
    health_unit::spawn_health_unit,
    defense_bar::initialise_defense_bar,
    defense_unit::spawn_defense_unit,
    roll_bar::initialise_roll_bar,
    roll_unit::spawn_roll_unit,
    consumable::spawn_consumable,
    defense::initialise_defense,
};

pub fn initialise_sprite_resource(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) -> SpriteResource {
    let sprite_resource = SpriteResource {
        sprite_sheet: sprite_sheet_handle,
    };

    world.add_resource(sprite_resource.clone());
    sprite_resource
}