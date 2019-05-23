use amethyst::ecs::prelude::{Component, DenseVecStorage};


#[derive(Clone)]
pub struct Explosion {
    pub duration: f32,
}

impl Component for Explosion {
    type Storage = DenseVecStorage<Self>;
}