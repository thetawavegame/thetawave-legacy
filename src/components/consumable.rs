use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConsumableComponent {
    pub name: String,
    #[serde(default = "des_speed")]
    pub speed: f32,
    #[serde(default)]
    pub health_value: f32,
    #[serde(default)]
    pub defense_value: f32,
    #[serde(default)]
    pub money_value: usize,
    pub armor_value: usize,
    pub sprite_index: usize,
}

fn des_speed() -> f32 {
    35.0
}

impl Component for ConsumableComponent {
    type Storage = DenseVecStorage<Self>;
}
