use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::components::Spawnable;

#[derive(Clone)]
pub struct Explosion {
    pub duration: f32,
    pub name: String,
}

impl Spawnable for Explosion {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn init(&mut self) {}
}

impl Component for Explosion {
    type Storage = DenseVecStorage<Self>;
}
