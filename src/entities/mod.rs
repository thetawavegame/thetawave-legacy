use crate::components::Animation;
use amethyst::{
    core::{math::Vector3, transform::Transform, Named},
    ecs::prelude::{Builder, Component, Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent},
};

pub mod blast;
pub mod boss;
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
    blast::fire_blast,
    boss::spawn_repeater,
    consumable::spawn_consumable,
    defense::initialize_defense,
    enemy::spawn_enemy,
    enemy_spawner::initialise_enemy_spawner,
    explosion::{spawn_blast_explosion, spawn_explosion},
    gamemaster::initialize_gamemaster,
    items::spawn_item,
    planet::initialize_planet,
    side_panel::initialize_side_panels,
    spaceship::initialize_spaceship,
    status_bar::initialize_status_bars,
    status_unit::spawn_status_unit,
    store::initialize_store,
};
use crate::components::Spawnable;

fn spawn_sprite_entity<T: Spawnable + Component + Send + Sync>(
    entities: &Entities,
    name: Named,
    sprite_render: SpriteRender,
    mut item_component: T,
    spawn_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let mut local_transform = Transform::default();
    local_transform.set_translation(spawn_position);

    println!("{} spawned!", name.name);
    item_component.init();

    lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(item_component)
        .with(local_transform)
        .with(Transparent)
        .with(name)
        .build();
}

fn spawn_animated_entity<T: Spawnable + Component + Send + Sync>(
    entities: &Entities,
    name: Named,
    sprite_render: SpriteRender,
    animation: Animation,
    mut item_component: T,
    spawn_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) -> Entity {
    let mut local_transform = Transform::default();
    local_transform.set_translation(spawn_position);

    println!("{} spawned!", name.name);
    item_component.init();

    let enemy_entity: Entity = entities.create();

    lazy_update.insert(enemy_entity, animation);
    lazy_update.insert(enemy_entity, sprite_render);
    lazy_update.insert(enemy_entity, item_component);
    lazy_update.insert(enemy_entity, local_transform);
    lazy_update.insert(enemy_entity, Transparent);
    lazy_update.insert(enemy_entity, name);

    enemy_entity
}
