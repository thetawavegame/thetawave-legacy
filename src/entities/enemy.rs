use amethyst::{
    ecs::prelude::{Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent, Flipped, },
    core::{
        transform::Transform,
        nalgebra::Vector3,
    }
};

use rand::seq::SliceRandom;

use crate::{
    components::{Pool, Enemy},
    resources::SpriteResource,
};


pub fn spawn_enemy(entities: &Entities, sprite_resource: &ReadExpect<SpriteResource>, enemy_pool: &mut Pool<Enemy>, spawn_position: Vector3<f32>, lazy_update: &ReadExpect<LazyUpdate>) {
    let enemy_entity: Entity = entities.create();

    let mut local_transform = Transform::default();
    local_transform.set_position(spawn_position);

    //get a random item from the enemy pool and then remove it from the pool
    let random_enemy_name = enemy_pool.spawn_list.choose(&mut rand::thread_rng()).cloned().unwrap();
    let random_enemy = enemy_pool.spawns[&random_enemy_name].clone();

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_resource.sprite_sheet.clone(),
        sprite_number: random_enemy.sprite_index,
    };

    lazy_update.insert(enemy_entity, sprite_render);
    lazy_update.insert(enemy_entity, random_enemy);
    lazy_update.insert(enemy_entity, local_transform);
    lazy_update.insert(enemy_entity, Transparent);
    lazy_update.insert(enemy_entity, Flipped::Vertical);

}

