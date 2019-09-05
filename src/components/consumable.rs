use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Consumable {
    #[serde(default = "des_width")]
    pub width: f32,
    #[serde(default = "des_height")]
    pub height: f32,
    #[serde(default = "des_hitbox_width")]
    pub hitbox_width: f32,
    #[serde(default = "des_hitbox_height")]
    pub hitbox_height: f32,
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

fn des_width() -> f32 { 12.0 }
fn des_height() -> f32 { 12.0 }
fn des_hitbox_width() -> f32 { 10.0 }
fn des_hitbox_height() -> f32 { 10.0 }
fn des_speed() -> f32 { 35.0 }

impl Component for Consumable {
    type Storage = DenseVecStorage<Self>;
}
