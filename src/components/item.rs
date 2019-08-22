use amethyst::ecs::prelude::{Component, DenseVecStorage};
use std::{
    collections::{HashMap},
};


#[derive(Clone)]
pub struct Item {
    pub width: f32,
    pub height: f32,
    pub hitbox_width: f32,
    pub hitbox_height: f32,
    pub stat_effects: HashMap<String, f32>,
    pub bool_effects: HashMap<String, bool>,
    pub sprite_index: usize,
    pub speed: f32,
}

impl Component for Item {
    type Storage = DenseVecStorage<Self>;
}