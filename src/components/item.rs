use amethyst::ecs::prelude::{Component, DenseVecStorage, NullStorage};
use std::{
    collections::{HashMap},
};

use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Item {
    #[serde(default)]
    pub width: f32,
    #[serde(default)]
    pub height: f32,
    #[serde(default)]
    pub hitbox_width: f32,
    #[serde(default)]
    pub hitbox_height: f32,
    #[serde(default)]
    pub speed: f32,
    #[serde(default)]
    pub stat_effects: HashMap<String, f32>,
    #[serde(default)]
    pub bool_effects: HashMap<String, bool>,
    // this field is required when deserializing
    pub sprite_index: usize,
}

impl Default for Item {
    fn default() -> Self {
        Item {
            width: 14.0,
            height: 14.0,
            hitbox_width: 4.0,
            hitbox_height: 4.0,
            speed: 70.0,
            stat_effects: Default::default(),
            bool_effects: Default::default(),
            sprite_index: 0
        }
    }
}

impl Component for Item {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct ItemSpawnerTag;

impl Component for ItemSpawnerTag {
    type Storage = NullStorage<Self>;
}