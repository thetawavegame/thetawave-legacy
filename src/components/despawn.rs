use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DespawnAtBorderComponent {
    pub top_offset: Option<f32>,
    pub bottom_offset: Option<f32>,
    pub left_offset: Option<f32>,
    pub right_offset: Option<f32>,
}

impl Component for DespawnAtBorderComponent {
    type Storage = DenseVecStorage<Self>;
}
