use amethyst::ecs::prelude::{Component, DenseVecStorage};


pub struct Spaceship {
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub fire_speed: f32,
    pub fire_reset_timer: f32
}

impl Component for Spaceship {
    type Storage = DenseVecStorage<Self>;
}