use amethyst::{
    ecs::prelude::{World, Builder},
    core::transform::Transform,
};

use crate::{
    components::Pool,
    components::{Spawner, Item},
};

use std::{
    collections::HashMap,
    vec::Vec,
};

use crate::space_shooter::{ARENA_MAX_Y, ARENA_MIN_X, ARENA_WIDTH};


const ITEM_HEIGHT: f32 = 14.0;
pub const ITEM_WIDTH: f32 = 14.0;
const ITEM_HITBOX_WIDTH: f32 = 4.0;
const ITEM_HITBOX_HEIGHT: f32 = 4.0;
const ITEM_SPEED: f32 = 70.0;
const SPAWN_INTERVAL: f32 = 10.0;

const STEEL_BARREL_SPRITE_INDEX: usize = 5;
const PLASMA_BLASTS_SPRITE_INDEX: usize = 6;
const HAZARDOUS_REACTOR_SPRITE_INDEX: usize = 7;
const WARP_THRUSTER_SPRITE_INDEX: usize = 8;

const SPAWNER_Y_OFFSET: f32 = 20.0;

pub fn initialise_item_spawner(world: &mut World) {

    let item_list: Vec<String> = vec![
        "steel_barrel".to_string(),
        "plasma_blasts".to_string(),
        "hazardous_reactor".to_string(),
        "warp_thruster".to_string(),
    ];

    let mut steel_barrel_bool_effects = HashMap::new();
    steel_barrel_bool_effects.insert("barrel_immunity".to_string(), true);

    let steel_barrel = Item {
        width: ITEM_WIDTH,
        height: ITEM_HEIGHT,
        hitbox_width: ITEM_HITBOX_WIDTH,
        hitbox_height: ITEM_HITBOX_HEIGHT,
        stat_effects: HashMap::new(),
        bool_effects: steel_barrel_bool_effects,
        sprite_index: STEEL_BARREL_SPRITE_INDEX,
        speed: ITEM_SPEED,
    };

    let mut plasma_blasts_stat_effects = HashMap::new();
    plasma_blasts_stat_effects.insert("fire_speed".to_string(), -0.05);
    plasma_blasts_stat_effects.insert("damage".to_string(), 5.0);

    let plasma_blasts = Item {
        width: ITEM_WIDTH,
        height: ITEM_HEIGHT,
        hitbox_width: ITEM_HITBOX_WIDTH,
        hitbox_height: ITEM_HITBOX_HEIGHT,
        stat_effects: plasma_blasts_stat_effects,
        bool_effects: HashMap::new(),
        sprite_index: PLASMA_BLASTS_SPRITE_INDEX,
        speed: ITEM_SPEED,
    };

    let mut hazardous_reactor_stat_effects = HashMap::new();
    hazardous_reactor_stat_effects.insert("max_speed".to_string(), 20.0);

    let hazardous_reactor = Item {
        width: ITEM_WIDTH,
        height: ITEM_HEIGHT,
        hitbox_width: ITEM_HITBOX_WIDTH,
        hitbox_height: ITEM_HITBOX_HEIGHT,
        stat_effects: hazardous_reactor_stat_effects,
        bool_effects: HashMap::new(),
        sprite_index: HAZARDOUS_REACTOR_SPRITE_INDEX,
        speed: ITEM_SPEED,
    };

    let mut warp_thruster_stat_effects = HashMap::new();
    warp_thruster_stat_effects.insert("acceleration".to_string(), 1.0);
    warp_thruster_stat_effects.insert("deceleration".to_string(), 1.0);

    let warp_thruster = Item {
        width: ITEM_WIDTH,
        height: ITEM_HEIGHT,
        hitbox_width: ITEM_HITBOX_WIDTH,
        hitbox_height: ITEM_HITBOX_HEIGHT,
        stat_effects: warp_thruster_stat_effects,
        bool_effects: HashMap::new(),
        sprite_index: WARP_THRUSTER_SPRITE_INDEX,
        speed: ITEM_SPEED,
    };

    let mut items = HashMap::new();
    items.insert("steel_barrel".to_string(), steel_barrel);
    items.insert("plasma_blasts".to_string(), plasma_blasts);
    items.insert("hazardous_reactor".to_string(), hazardous_reactor);
    items.insert("warp_thruster".to_string(), warp_thruster);

    let item_pool = Pool {
        spawn_list: item_list,
        spawns: items,
    };

    let mut local_transform = Transform::default();
    local_transform.set_xyz(ARENA_MIN_X + (ARENA_WIDTH / 2.0), ARENA_MAX_Y + SPAWNER_Y_OFFSET, 0.0);

    world
        .create_entity()
        .with(Spawner {
            pool: item_pool,
            spawn_interval: SPAWN_INTERVAL,
            spawn_timer: SPAWN_INTERVAL,
            spawn_counter: 0,
        })
        .with(local_transform)
        .build();

}

