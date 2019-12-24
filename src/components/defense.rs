use crate::components::Living;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Defense {
    pub max_defense: f32,
    pub defense: f32,
}

impl Living for Defense {
    fn health(&self) -> f32 {
        self.defense
    }
    fn max_health(&self) -> f32 {
        self.max_defense
    }
    fn set_health(&mut self, value: f32) {
        self.defense = value;
    }
    fn set_max_health(&mut self, value: f32) {
        self.max_defense = value;
    }
}

impl Component for Defense {
    type Storage = DenseVecStorage<Self>;
}
