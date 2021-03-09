use amethyst::ecs::prelude::{Component, DenseVecStorage};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ColorChannelChange {
    pub delta_value: f32,
    pub value: f32,
    pub min_value: f32,
    pub max_value: f32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FadeComponent {
    pub red_change: Option<ColorChannelChange>,
    pub green_change: Option<ColorChannelChange>,
    pub blue_change: Option<ColorChannelChange>,
    pub alpha_change: Option<ColorChannelChange>,
}

impl Component for FadeComponent {
    type Storage = DenseVecStorage<Self>;
}
