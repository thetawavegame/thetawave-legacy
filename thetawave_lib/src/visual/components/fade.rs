use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};

/// Manages change in data for a single color channel
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ColorChannelChange {
    /// Change in color channel per frame
    pub delta_value: f32,
    /// Current color channel value
    pub value: f32,
    /// Minimum value for color channel
    pub min_value: f32,
    /// Maximum value for color channel
    pub max_value: f32,
}

/// Used for fading a sprite/animation
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FadeComponent {
    /// Red channel fade data
    pub red_change: Option<ColorChannelChange>,
    /// Green channel fade data
    pub green_change: Option<ColorChannelChange>,
    /// Blue channel fade data
    pub blue_change: Option<ColorChannelChange>,
    /// Alpha channel fade data
    pub alpha_change: Option<ColorChannelChange>,
}

impl Component for FadeComponent {
    type Storage = DenseVecStorage<Self>;
}
