use amethyst::ecs::prelude::{Component, DenseVecStorage};


pub struct EnemySpawner{
    pub spawn_timer: f32,
    pub spawn_interval: f32,
    pub enemies_spawned: u32,
}

impl Component for EnemySpawner {
    type Storage = DenseVecStorage<Self>;
}