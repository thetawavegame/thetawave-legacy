use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Consumable {
    #[serde(default)]
    pub width: f32,
    #[serde(default)]
    pub height: f32,
    #[serde(default)]
    pub hitbox_width: f32,
    #[serde(default)]
    pub hitbox_height: f32,
    #[serde(default)]
    pub health_value: f32,
    #[serde(default)]
    pub defense_value: f32,
    #[serde(default)]
    pub money_value: usize,
    #[serde(default)]
    pub speed: f32,
    // this value must be set during deserialization
    pub sprite_index: usize,
}

impl Default for Consumable {
    fn default() -> Self {
        Consumable {
            width: 12.0,
            height: 12.0,
            hitbox_width: 10.0,
            hitbox_height: 10.0,
            health_value: 0.0,
            defense_value: 0.0,
            money_value: 0,
            speed: 35.0,
            sprite_index: 0,
        }
    }
}

impl Component for Consumable {
    type Storage = DenseVecStorage<Self>;
}
