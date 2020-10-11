use amethyst::ecs::prelude::{Component, DenseVecStorage, NullStorage};

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ItemComponent {
    #[serde(default = "des_price")]
    pub price: usize,
    #[serde(default)]
    pub stat_effects: HashMap<String, f32>,
    #[serde(default)]
    pub bool_effects: HashMap<String, bool>,
    pub sprite_index: usize,
    pub name: String,
}

fn des_price() -> usize {
    10
}

impl Component for ItemComponent {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct ItemSpawnerTag;

impl Component for ItemSpawnerTag {
    type Storage = NullStorage<Self>;
}
