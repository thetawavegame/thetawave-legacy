use amethyst::{
    prelude::Builder,
    ecs::prelude::World,
    core::transform::Transform,
};

use crate::{
    components::{Spawner, Enemy, EnemyType},
    components::{Pool, Consumable},
};

use std::{
    collections::HashMap,
    vec::Vec,
};
use crate::space_shooter::{ARENA_MAX_Y, ARENA_MIN_X, ARENA_WIDTH};


const CONSUMABLE_WIDTH: f32 = 12.0;
const CONSUMABLE_HEIGHT: f32 = 12.0;
const CONSUMABLE_HITBOX_WIDTH: f32 = 10.0;
const CONSUMABLE_HITBOX_HEIGHT: f32 = 10.0;
const CONSUMABLE_HEALTH_VALUE: f32 = 30.0;
const CONSUMABLE_DEFENSE_VALUE: f32 = 70.0;
const CONSUMABLE_SPEED: f32 = 35.0;
const CONSUMABLE_HEALTH_RATIO: usize = 5;
const CONSUMABLE_DEFENSE_RATIO: usize = 1;

const ENEMY_HEIGHT: f32 = 18.0;
const ENEMY_WIDTH: f32 = 18.0;
const ENEMY_FIRE_SPEED: f32 = 3.0;
const ENEMY_HEALTH: f32 = 110.0; //change back to 100
const ENEMY_HITBOX_WIDTH: f32 = 14.0;
const ENEMY_HITBOX_HEIGHT: f32 = 14.0;
const SPAWN_INTERVAL: f32 = 1.5;
const ENEMY_FIRE_DELAY: f32 = 1.0;
const ENEMY_DEFENSE_DAMAGE: f32 = 50.0;
const ENEMY_MAX_SPEED: f32 = 40.0;
const ENEMY_ACCELERATION_X: f32 = 2.0;
const ENEMY_DECELERATION_X: f32 = 1.0;
const ENEMY_ACCELERATION_Y: f32 = 4.0;
const ENEMY_DECELERATION_Y: f32 = 1.0;
const ENEMY_MAX_KNOCKBACK_SPEED: f32 = 100.0;
const ENEMY_COLLISION_DAMAGE: f32 = 30.0;
const ENEMY_DROP_CHANCE: f32 = 0.09;
const ENEMY_BLAST_SPEED: f32 = -60.0;
const ENEMY_BLAST_DAMAGE: f32 = 30.0;

const ENEMY_PAWN_RATIO: usize = 50;
const ENEMY_DRONE_RATIO: usize = 100;
const ENEMY_HAULER_RATIO: usize = 5;

const PAWN_SPRITE_INDEX: usize = 1;
const DRONE_SPRITE_INDEX: usize = 2;
const HAULER_SPRITE_INDEX: usize = 15;

const HEALTH_CONSUMABLE_SPRITE_INDEX: usize = 13;
const DEFENSE_CONSUMABLE_SPRITE_INDEX: usize = 14;

const SPAWNER_Y_OFFSET: f32 = 20.0;


pub fn initialise_enemy_spawner(world: &mut World) {

    //create consumables
    let mut consumables_list: Vec<String> = vec![];

    for _ in 0..CONSUMABLE_HEALTH_RATIO {
        consumables_list.push("health".to_string());
    }
    for _ in 0..CONSUMABLE_DEFENSE_RATIO {
        consumables_list.push("defense".to_string());
    }

    let health_consumable = Consumable {
        width: CONSUMABLE_WIDTH,
        height: CONSUMABLE_HEIGHT,
        hitbox_width: CONSUMABLE_HITBOX_WIDTH,
        hitbox_height: CONSUMABLE_HITBOX_HEIGHT,
        health_value: CONSUMABLE_HEALTH_VALUE,
        defense_value: 0.0,
        sprite_index: HEALTH_CONSUMABLE_SPRITE_INDEX,
        speed: CONSUMABLE_SPEED,
    };

    let defense_consumable = Consumable {
        width: CONSUMABLE_WIDTH,
        height: CONSUMABLE_HEIGHT,
        hitbox_width: CONSUMABLE_HITBOX_WIDTH,
        hitbox_height: CONSUMABLE_HITBOX_HEIGHT,
        health_value: 0.0,
        defense_value: CONSUMABLE_DEFENSE_VALUE,
        sprite_index: DEFENSE_CONSUMABLE_SPRITE_INDEX,
        speed: CONSUMABLE_SPEED,
    };

    let mut consumables = HashMap::new();
    consumables.insert("health".to_string(), health_consumable);
    consumables.insert("defense".to_string(), defense_consumable);

    //create consumable pools for enemies
    let standard_pool = Pool {
        spawn_list: consumables_list,
        spawns: consumables,
    };


    let mut enemy_list: Vec<String> = vec![];

    for _ in 0..ENEMY_PAWN_RATIO {
        enemy_list.push("pawn".to_string());
    }
    for _ in 0..ENEMY_DRONE_RATIO {
        enemy_list.push("drone".to_string());
    }
    for _ in 0..ENEMY_HAULER_RATIO {
        enemy_list.push("hauler".to_string());
    }

    let pawn = Enemy {
        enemy_type: EnemyType::Pawn,
        width: ENEMY_WIDTH,
        height: ENEMY_HEIGHT,
        fire_speed: ENEMY_FIRE_SPEED,
        health: ENEMY_HEALTH,
        hitbox_width: ENEMY_HITBOX_WIDTH,
        hitbox_height: ENEMY_HITBOX_HEIGHT,
        sprite_index: PAWN_SPRITE_INDEX,
        fires: true,
        fire_reset_timer: ENEMY_FIRE_DELAY,
        blast_speed: ENEMY_BLAST_SPEED,
        blast_damage: ENEMY_BLAST_DAMAGE,
        defense_damage: ENEMY_DEFENSE_DAMAGE,
        max_speed: ENEMY_MAX_SPEED - 10.0,
        current_velocity_x: 0.0,
        current_velocity_y: (-1.0)*(ENEMY_MAX_SPEED- 10.0),
        acceleration_x: ENEMY_ACCELERATION_X,
        deceleration_x: ENEMY_DECELERATION_X,
        acceleration_y: ENEMY_ACCELERATION_Y,
        deceleration_y: ENEMY_DECELERATION_Y,
        knockback_max_speed: ENEMY_MAX_KNOCKBACK_SPEED,
        collision_damage: ENEMY_COLLISION_DAMAGE,
        consumable_pool: standard_pool.clone(),
        drop_chance: ENEMY_DROP_CHANCE,
    };

    let drone = Enemy {
        enemy_type: EnemyType::Drone,
        width: ENEMY_WIDTH,
        height: ENEMY_HEIGHT,
        fire_speed: ENEMY_FIRE_SPEED,
        health: ENEMY_HEALTH - 50.0,
        hitbox_width: ENEMY_HITBOX_WIDTH,
        hitbox_height: ENEMY_HITBOX_HEIGHT,
        sprite_index: DRONE_SPRITE_INDEX,
        fires: false,
        fire_reset_timer: 0.0,
        blast_speed: 0.0,
        blast_damage: 0.0,
        defense_damage: ENEMY_DEFENSE_DAMAGE - 20.0,
        max_speed: ENEMY_MAX_SPEED,
        current_velocity_x: 0.0,
        current_velocity_y: (-1.0)*ENEMY_MAX_SPEED,
        acceleration_x: ENEMY_ACCELERATION_X,
        deceleration_x: ENEMY_DECELERATION_X,
        acceleration_y: ENEMY_ACCELERATION_Y,
        deceleration_y: ENEMY_DECELERATION_Y,
        knockback_max_speed: ENEMY_MAX_KNOCKBACK_SPEED,
        collision_damage: ENEMY_COLLISION_DAMAGE,
        consumable_pool: standard_pool.clone(),
        drop_chance: ENEMY_DROP_CHANCE,
    };

    let hauler = Enemy {
        enemy_type: EnemyType::Hauler,
        width: ENEMY_WIDTH,
        height: ENEMY_HEIGHT * 2.0,
        fire_speed: ENEMY_FIRE_SPEED,
        health: ENEMY_HEALTH * 2.0,
        hitbox_width: ENEMY_HITBOX_WIDTH,
        hitbox_height: ENEMY_HITBOX_HEIGHT * 2.0,
        sprite_index: HAULER_SPRITE_INDEX,
        fires: false,
        fire_reset_timer: ENEMY_FIRE_DELAY,
        blast_speed: ENEMY_BLAST_SPEED,
        blast_damage: ENEMY_BLAST_DAMAGE,
        defense_damage: -2.0 * ENEMY_DEFENSE_DAMAGE,
        max_speed: ENEMY_MAX_SPEED - 15.0,
        current_velocity_x: 0.0,
        current_velocity_y: (-1.0)*(ENEMY_MAX_SPEED- 20.0),
        acceleration_x: ENEMY_ACCELERATION_X,
        deceleration_x: ENEMY_DECELERATION_X,
        acceleration_y: ENEMY_ACCELERATION_Y,
        deceleration_y: ENEMY_DECELERATION_Y,
        knockback_max_speed: ENEMY_MAX_KNOCKBACK_SPEED,
        collision_damage: ENEMY_COLLISION_DAMAGE,
        consumable_pool: standard_pool.clone(),
        drop_chance: 0.0,
    };

    let mut enemies = HashMap::new();
    enemies.insert("pawn".to_string(), pawn);
    enemies.insert("drone".to_string(), drone);
    enemies.insert("hauler".to_string(), hauler);

    let enemy_pool = Pool {
        spawn_list: enemy_list,
        spawns: enemies,
    };

    //create transform
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_MIN_X + (ARENA_WIDTH / 2.0), ARENA_MAX_Y + SPAWNER_Y_OFFSET, 0.0);

    world
        .create_entity()
        .with(Spawner {
            pool: enemy_pool,
            spawn_timer: 0.0,
            spawn_interval: SPAWN_INTERVAL,
            spawn_counter: 0,
        })
        .with(local_transform)
        .build();
}