use crate::entities::SpawnableType;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ItemComponent {
    pub price: usize,
    #[serde(default)]
    pub stat_effects: HashMap<String, f32>,
    #[serde(default)]
    pub bool_effects: HashMap<String, bool>,
    pub spawnable_type: SpawnableType,
}

impl Component for ItemComponent {
    type Storage = DenseVecStorage<Self>;
}
