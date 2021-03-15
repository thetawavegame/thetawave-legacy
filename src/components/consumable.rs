use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};

use crate::entities::SpawnableType;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConsumableComponent {
    pub sound_effect: String,
    #[serde(default)]
    pub health_value: f32,
    #[serde(default)]
    pub defense_value: f32,
    #[serde(default)]
    pub money_value: usize,
    #[serde(default)]
    pub armor_value: usize,
    pub price: usize,
    pub spawnable_type: SpawnableType,
}

impl Component for ConsumableComponent {
    type Storage = DenseVecStorage<Self>;
}
