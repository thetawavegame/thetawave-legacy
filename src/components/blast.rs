use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Clone)]
pub struct Blast {
    pub speed: f32,
    pub hitbox_radius: f32,
    pub damage: f32,
    pub poison_damage: f32,
    pub x_velocity: f32,
    pub y_velocity: f32,
    pub velocity_factor: f32,
    pub allied: bool,
}

impl Component for Blast {
    type Storage = DenseVecStorage<Self>;
}