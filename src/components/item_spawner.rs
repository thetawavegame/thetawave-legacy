use amethyst::ecs::prelude::{Component, DenseVecStorage};
use std::{
    collections::{HashMap},
    vec::Vec,
};
use crate::components::Item;


/*
//Item pool info
let mut stat_effects = HashMap::new();
stat_effects.insert(
    "barrel_damage",
    60.0
);
*/

pub struct ItemPool {
    pub available_items: Vec<String>,
    //pub item_stat_effects: HashMap<String, HashMap<String, f32>>,
    pub items: HashMap<String, Item>
}

/*
impl Component for ItemPool {
    type Storage = DenseVecStorage<Self>;
}
*/

pub struct ItemSpawner {
    pub item_pool: ItemPool,
    pub spawn_timer: f32,
    pub spawn_interval: f32,
}

impl Component for ItemSpawner {
    type Storage = DenseVecStorage<Self>;
}