use crate::entities::EntityType;
use amethyst::ecs::prelude::{Component, DenseVecStorage, NullStorage};

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ItemComponent {
    pub price: usize,
    #[serde(default)]
    pub stat_effects: HashMap<String, f32>,
    #[serde(default)]
    pub bool_effects: HashMap<String, bool>,
    pub entity_type: EntityType,
}

impl Component for ItemComponent {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct ItemSpawnerTag;

impl Component for ItemSpawnerTag {
    type Storage = NullStorage<Self>;
}
