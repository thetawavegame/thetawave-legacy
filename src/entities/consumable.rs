use amethyst::{
    ecs::prelude::{Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent},
    core::{
        transform::Transform,
        math::Vector3,
    }
};

use rand::seq::SliceRandom;

use crate::{
    components::{Pool, Consumable},
    resources::SpriteResource,
};


pub fn spawn_consumable(entities: &Entities, sprite_resource: &ReadExpect<SpriteResource>, consumable_pool: &mut Pool<Consumable>, spawn_position: Vector3<f32>, lazy_update: &ReadExpect<LazyUpdate>) {
    let consumable_entity: Entity = entities.create();

    let mut local_transform = Transform::default();
    local_transform.set_translation(spawn_position);

    //get a random item from the enemy pool and then remove it from the pool
    let random_consumable_name = consumable_pool.spawn_list.choose(&mut rand::thread_rng()).cloned().unwrap();
    let random_consumable = consumable_pool.spawns[&random_consumable_name].clone();

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_resource.sprite_sheet.clone(),
        sprite_number: random_consumable.sprite_index,
    };

    lazy_update.insert(consumable_entity, sprite_render);
    lazy_update.insert(consumable_entity, random_consumable);
    lazy_update.insert(consumable_entity, local_transform);
    lazy_update.insert(consumable_entity, Transparent);

}