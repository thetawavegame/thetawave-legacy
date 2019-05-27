use amethyst::{
    prelude::Builder,
    ecs::prelude::World,
    core::transform::Transform,
};

use crate::{
    components::{EnemySpawner, Enemy, EnemyPool},
    space_shooter::{GAME_HEIGHT, GAME_WIDTH},
};

use std::{
    collections::HashMap,
    vec::Vec,
};

const ENEMY_HEIGHT: f32 = 18.0;
pub const ENEMY_WIDTH: f32 = 18.0;
const ENEMY_SPEED: f32 = 40.0;
const ENEMY_FIRE_SPEED: f32 = 0.5;
const ENEMY_HEALTH: f32 = 100.0;
const ENEMY_HITBOX_WIDTH: f32 = 14.0;
const ENEMY_HITBOX_HEIGHT: f32 = 14.0;
const SPAWN_INTERVAL: f32 = 1.0;


pub fn initialise_enemy_spawner(world: &mut World) {

    let enemy_list: Vec<String> = vec![
        "drone".to_string(),
    ];

    let drone = Enemy {
        width: ENEMY_WIDTH,
        height: ENEMY_HEIGHT,
        speed: ENEMY_SPEED,
        fire_speed: ENEMY_FIRE_SPEED,
        health: ENEMY_HEALTH,
        hitbox_width: ENEMY_HITBOX_WIDTH,
        hitbox_height: ENEMY_HITBOX_HEIGHT,
        barrel_damaged: false,
        sprite_index: 2,
    };

    let mut enemies = HashMap::new();
    enemies.insert("drone".to_string(), drone);

    let enemy_pool = EnemyPool {
        available_enemies: enemy_list,
        enemies: enemies,
    };

    //create transform
    let mut local_transform = Transform::default();
    local_transform.set_xyz(GAME_WIDTH / 2.0, GAME_HEIGHT, 0.0);

    world
        .create_entity()
        .with(EnemySpawner {
            enemy_pool: enemy_pool,
            spawn_timer: 0.0,
            spawn_interval: SPAWN_INTERVAL,
            enemies_spawned: 0,
        })
        .with(local_transform)
        .build();
}