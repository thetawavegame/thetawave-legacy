use amethyst::{
    ecs::prelude::{World, Builder},
    core::transform::Transform,
};

use crate::{
    components::ItemPool,
    components::{ItemSpawner, Item},
    space_shooter::{GAME_WIDTH, GAME_HEIGHT},
};

use std::{
    collections::HashMap,
    vec::Vec,
};

//const ITEM_POOL_LENGTH: usize = 1;
const ITEM_HEIGHT: f32 = 14.0;
pub const ITEM_WIDTH: f32 = 14.0;
const ITEM_HITBOX_WIDTH: f32 = 4.0;
const ITEM_HITBOX_HEIGHT: f32 = 4.0;
const ITEM_SPEED: f32 = 50.0;

const SPAWN_INTERVAL: f32 = 10.0;

pub fn initialise_item_spawner(world: &mut World) {

    let item_list: Vec<String> = vec![
        "steel_barrel".to_string()
    ];

    let mut steel_barrel_stat_effects = HashMap::new();
    steel_barrel_stat_effects.insert("barrel_damage".to_string(), 60.0);

    let steel_barrel = Item {
        width: ITEM_WIDTH,
        height: ITEM_HEIGHT,
        hitbox_width: ITEM_HITBOX_WIDTH,
        hitbox_height: ITEM_HITBOX_HEIGHT,
        stat_effects: steel_barrel_stat_effects,
        sprite_index: 4,
        speed: ITEM_SPEED,
    };

    let mut items = HashMap::new();
    items.insert("steel_barrel".to_string(), steel_barrel);

    let mut local_transform = Transform::default();
    local_transform.set_xyz(GAME_WIDTH / 2.0, GAME_HEIGHT, 0.0);

    let item_pool = ItemPool {
        available_items: item_list,
        items: items,
    };

    world
        .create_entity()
        .with(ItemSpawner {
            item_pool: item_pool,
            spawn_interval: SPAWN_INTERVAL,
            spawn_timer: SPAWN_INTERVAL,
        })
        .with(local_transform)
        .build();

}

