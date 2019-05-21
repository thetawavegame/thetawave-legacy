use amethyst::ecs::prelude::{Component, DenseVecStorage};


#[derive(Clone)]
pub struct Blast {
    pub speed: f32,
}

impl Component for Blast {
    type Storage = DenseVecStorage<Self>;
}