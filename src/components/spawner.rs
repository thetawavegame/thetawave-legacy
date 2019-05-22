use amethyst::ecs::prelude::{Component, DenseVecStorage};


pub struct Spawner{
    pub spawn_timer: f32,
    pub spawn_interval: f32,
}

impl Component for Spawner {
    type Storage = DenseVecStorage<Self>;
}