use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{Builder, Component, Entities, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent},
};

pub mod blast;
pub mod consumable;
pub mod defense;
pub mod enemy;
pub mod enemy_spawner;
pub mod explosion;
pub mod gamemaster;
pub mod items;
pub mod planet;
pub mod side_panel;
pub mod spaceship;
pub mod status_bar;
pub mod status_unit;
pub mod store;

pub use self::{
    blast::fire_blast, consumable::spawn_consumable, defense::initialise_defense,
    enemy::spawn_enemy, enemy_spawner::initialise_enemy_spawner, explosion::spawn_explosion,
    gamemaster::initialise_gamemaster, items::spawn_item, planet::initialise_planet,
    side_panel::initialise_side_panels, spaceship::initialise_spaceship,
    status_bar::initialise_status_bars, status_unit::spawn_status_unit, store::initialise_store,
};
use crate::components::Spawnable;

fn spawn_sprite_entity<T: Spawnable + Component + Send + Sync>(
    entities: &Entities,
    sprite_render: SpriteRender,
    mut item_comp: T,
    spawn_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let mut local_transform = Transform::default();
    local_transform.set_translation(spawn_position);

    println!("{} spawned!", item_comp.name());
    item_comp.init();

    lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(item_comp)
        .with(local_transform)
        .with(Transparent)
        .build();
}
