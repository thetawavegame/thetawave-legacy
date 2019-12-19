use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{Builder, Component, Entities, LazyUpdate, ReadExpect, Entity},
    renderer::{SpriteRender, Transparent},
};
use crate::{
    components::Animation,
};

pub mod blast;
pub mod spaceship;
pub mod enemy;
pub mod enemy_spawner;
pub mod explosion;
pub mod items;
pub mod side_panel;
pub mod consumable;
pub mod defense;
pub mod status_bar;
pub mod status_unit;
pub mod gamemaster;
pub mod store;
pub mod planet;
pub mod boss;

pub use self::{
    blast::fire_blast,
    spaceship::initialise_spaceship,
    enemy::{spawn_enemy},
    enemy_spawner::{initialise_enemy_spawner },
    explosion::{spawn_explosion, spawn_blast_explosion},
    items::spawn_item,
    side_panel::initialise_side_panels,
    consumable::spawn_consumable,
    defense::initialise_defense,
    status_bar::initialise_status_bars,
    status_unit::spawn_status_unit,
    gamemaster::initialise_gamemaster,
    store::{initialise_store},
    planet::{initialise_planet},
    boss::{spawn_repeater},
};
use crate::{
    components::{ Spawnable },
};

fn spawn_sprite_entity<T: Spawnable + Component + Send + Sync>(
    entities: &Entities,
    sprite_render: SpriteRender,
    mut item_component: T,
    spawn_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let mut local_transform = Transform::default();
    local_transform.set_translation(spawn_position);

    println!("{} spawned!", item_component.name());
    item_component.init();

    lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(item_component)
        .with(local_transform)
        .with(Transparent)
        .build();
}

fn spawn_animated_entity<T: Spawnable + Component + Send + Sync>(
    entities: &Entities,
    sprite_render: SpriteRender,
    animation: Animation,
    mut item_component: T,
    spawn_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) -> Entity {
    let mut local_transform = Transform::default();
    local_transform.set_translation(spawn_position);

    println!("{} spawned!", item_component.name());
    item_component.init();

    let enemy_entity: Entity = entities.create();

    /*
    lazy_update
        .create_entity(entities)
        .with(animation)
        .with(sprite_render)
        .with(item_component)
        .with(local_transform)
        .with(Transparent)
        .build();
        */

    lazy_update.insert(enemy_entity, animation);
    lazy_update.insert(enemy_entity, sprite_render);
    lazy_update.insert(enemy_entity, item_component);
    lazy_update.insert(enemy_entity, local_transform);
    lazy_update.insert(enemy_entity, Transparent);

    enemy_entity
}
