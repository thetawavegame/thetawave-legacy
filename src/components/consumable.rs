use amethyst::ecs::prelude::{Component, DenseVecStorage};


#[derive(Clone)]
pub struct Consumable {
    pub width: f32,
    pub height: f32,
    pub hitbox_width: f32,
    pub hitbox_height: f32,
    pub health_value: f32,
    pub defense_value: f32,
    pub sprite_index: usize,
    pub speed: f32,
}

impl Component for Consumable {
    type Storage = DenseVecStorage<Self>;
}