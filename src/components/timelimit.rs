use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::components::Spawnable;

#[derive(Clone)]
pub struct TimeLimitComponent {
    pub duration: f32
}

impl Spawnable for TimeLimitComponent {
  fn init(&mut self) {}
}

impl Component for TimeLimitComponent {
    type Storage = DenseVecStorage<Self>;
}
