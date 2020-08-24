use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};

use crate::components::Spawnable;

#[derive(Clone, Serialize, Deserialize)]
pub struct Consumable {
    pub name: String,
    #[serde(default = "des_width")]
    pub width: f32,
    #[serde(default = "des_height")]
    pub height: f32,
    #[serde(default = "des_hitbox_width")]
    pub hitbox_width: f32,
    #[serde(default = "des_hitbox_height")]
    pub hitbox_height: f32,
    #[serde(default = "des_hitbox_x_offset")]
    pub hitbox_x_offset: f32,
    #[serde(default = "des_hitbox_y_offset")]
    pub hitbox_y_offset: f32,
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
fn des_hitbox_width() -> f32 {
    10.0
}
fn des_hitbox_height() -> f32 {
    10.0
}
fn des_hitbox_x_offset() -> f32 {
    0.0
}
fn des_hitbox_y_offset() -> f32 {
    0.0
}
fn des_speed() -> f32 {
    35.0
}

impl Spawnable for Consumable {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn init(&mut self) {}
}

impl Component for Consumable {
    type Storage = DenseVecStorage<Self>;
}
