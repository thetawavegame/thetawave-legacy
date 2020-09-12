use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Consumable {
    pub name: String,
    #[serde(default = "des_width")]
    pub width: f32,
    #[serde(default = "des_height")]
    pub height: f32,
    #[serde(default = "des_speed")]
    pub speed: f32,
    #[serde(default)]
    pub health_value: f32,
    #[serde(default)]
    pub defense_value: f32,
    #[serde(default)]
    pub money_value: usize,
    pub sprite_index: usize,
}

fn des_width() -> f32 {
    12.0
}
fn des_height() -> f32 {
    12.0
}
fn des_speed() -> f32 {
    35.0
}

impl Component for Consumable {
    type Storage = DenseVecStorage<Self>;
}
