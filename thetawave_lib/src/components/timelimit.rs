use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimeLimitComponent {
    pub duration: f32,
}

impl Component for TimeLimitComponent {
    type Storage = DenseVecStorage<Self>;
}
