use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Clone)]
pub struct TimeLimitComponent {
    pub duration: f32,
}

impl Component for TimeLimitComponent {
    type Storage = DenseVecStorage<Self>;
}
