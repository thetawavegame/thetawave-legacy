use amethyst::{
    ecs::prelude::{Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent},
    core::{
        transform::Transform,
        math::Vector3,
    }
};

use crate::{
    resources::SpriteResource,
    components::{Pool, Item},
};

use rand::seq::SliceRandom;


pub fn spawn_item(entities: &Entities, item_resource: &ReadExpect<SpriteResource>, item_pool: &mut Pool<Item>, spawn_position: Vector3<f32>, lazy_update: &ReadExpect<LazyUpdate>) {
    let item_entity: Entity = entities.create();

    let mut local_transform = Transform::default();
    local_transform.set_translation(spawn_position);

    //get a random item from the item pool and then remove it from the pool
    let random_item_name = item_pool.spawn_list.choose(&mut rand::thread_rng()).cloned().unwrap();
    let random_item = item_pool.spawns[&random_item_name].clone();
    item_pool.spawn_list.retain(|x| x != &random_item_name);

    let sprite_render = SpriteRender {
        sprite_sheet: item_resource.sprite_sheet.clone(),
        sprite_number: random_item.sprite_index,
    };

    lazy_update.insert(item_entity, sprite_render);
    lazy_update.insert(item_entity, random_item);
    lazy_update.insert(item_entity, local_transform);
    lazy_update.insert(item_entity, Transparent);
}