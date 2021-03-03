use amethyst::ecs::prelude::{Component, DenseVecStorage};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct OpaqueFadeComponent {
    pub color_change: f32,
    pub max_color_value: f32,
    pub min_color_value: f32,
    pub current_color_value: f32,
}

impl Component for OpaqueFadeComponent {
    type Storage = DenseVecStorage<Self>;
}
