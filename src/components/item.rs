use amethyst::ecs::prelude::{Component, DenseVecStorage, NullStorage};

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Item {
    #[serde(default = "des_width")]
    pub width: f32,
    #[serde(default = "des_height")]
    pub height: f32,
    #[serde(default = "des_speed")]
    pub speed: f32,
    #[serde(default = "des_price")]
    pub price: usize,
    #[serde(default)]
    pub stat_effects: HashMap<String, f32>,
    #[serde(default)]
    pub bool_effects: HashMap<String, bool>,
    pub sprite_index: usize,
    pub name: String,
}

fn des_width() -> f32 {
    14.0
}
fn des_height() -> f32 {
    14.0
}
fn des_speed() -> f32 {
    70.0
}
fn des_price() -> usize {
    10
}

impl Component for Item {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct ItemSpawnerTag;

impl Component for ItemSpawnerTag {
    type Storage = NullStorage<Self>;
}
