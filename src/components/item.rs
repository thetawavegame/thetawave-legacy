use amethyst::ecs::prelude::{Component, DenseVecStorage};
use std::collections::HashMap;


pub struct Item {
    pub width: f32,
    pub height: f32,
    pub hitbox_width: f32,
    pub hitbox_height: f32,
    pub stat_effects: HashMap<&'static str, f32>,
}

impl Component for Item {
    type Storage = DenseVecStorage<Self>;
}

