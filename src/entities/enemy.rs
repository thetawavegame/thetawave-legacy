use amethyst::{
    ecs::prelude::{Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent, Flipped},
    core::{
        transform::Transform,
        nalgebra::Vector3,
    }
};

use rand::seq::SliceRandom;

use crate::{
    components::{Enemy, EnemySpawner, EnemyPool},
    resources::SpriteResource,
};


const ENEMY_HEIGHT: f32 = 18.0;
pub const ENEMY_WIDTH: f32 = 18.0;
const ENEMY_SPEED: f32 = 40.0;
const ENEMY_FIRE_SPEED: f32 = 0.5;
const ENEMY_HEALTH: f32 = 100.0;
const ENEMY_HITBOX_WIDTH: f32 = 14.0;
const ENEMY_HITBOX_HEIGHT: f32 = 14.0;

pub fn spawn_enemy(entities: &Entities, sprite_resource: &ReadExpect<SpriteResource>, enemy_pool: &mut EnemyPool, spawn_position: Vector3<f32>, lazy_update: &ReadExpect<LazyUpdate>) {
    let enemy_entity: Entity = entities.create();

    let mut local_transform = Transform::default();
    local_transform.set_position(spawn_position);

    //get a random item from the enemy pool and then remove it from the pool
    let random_enemy_name = enemy_pool.available_enemies.choose(&mut rand::thread_rng()).cloned().unwrap();
    let random_enemy = enemy_pool.enemies[&random_enemy_name].clone();

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

