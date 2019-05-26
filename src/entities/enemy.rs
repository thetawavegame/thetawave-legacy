use amethyst::{
    ecs::prelude::{Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent, Flipped},
    core::{
        transform::Transform,
        nalgebra::Vector3,
    }
};

use crate::{
    components::{Enemy, EnemySpawner},
    resources::SpriteResource,
};


const ENEMY_HEIGHT: f32 = 18.0;
pub const ENEMY_WIDTH: f32 = 18.0;
const ENEMY_SPEED: f32 = 40.0;
const ENEMY_FIRE_SPEED: f32 = 0.5;
const ENEMY_HEALTH: f32 = 100.0;
const ENEMY_HITBOX_WIDTH: f32 = 14.0;
const ENEMY_HITBOX_HEIGHT: f32 = 14.0;


pub fn spawn_enemy(entities: &Entities, enemy_resource: &ReadExpect<SpriteResource>, sprite_number: usize, spawn_position: Vector3<f32>, enemy_spawner: &mut EnemySpawner, lazy_update: &ReadExpect<LazyUpdate>) {
    let enemy_entity: Entity = entities.create();

    let mut local_transform = Transform::default();
    local_transform.set_position(spawn_position);

    let sprite_render = SpriteRender {
        sprite_sheet: enemy_resource.sprite_sheet.clone(),
        sprite_number: sprite_number,
    };

    lazy_update.insert(enemy_entity, sprite_render);
    lazy_update.insert(enemy_entity, Enemy {
        height: ENEMY_HEIGHT,
        width: ENEMY_WIDTH,
        speed: ENEMY_SPEED,
        fire_speed: ENEMY_FIRE_SPEED,
        health: ENEMY_HEALTH,
        hitbox_height: ENEMY_HITBOX_HEIGHT,
        hitbox_width: ENEMY_HITBOX_WIDTH,
    });
    lazy_update.insert(enemy_entity, local_transform);
    lazy_update.insert(enemy_entity, Transparent);
    lazy_update.insert(enemy_entity, Flipped::Vertical);

    enemy_spawner.enemies_spawned += 1;
    println!("Enemies spawned: {}", enemy_spawner.enemies_spawned);

}