use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};

/// Used for despawning entities at the arena borders
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DespawnAtBorderComponent {
    /// Y distance outside the arena's top border
    pub top_offset: Option<f32>,
    /// Y distance outside the arena's bottom border
    pub bottom_offset: Option<f32>,
    /// Y distance outside the arena's left border
    pub left_offset: Option<f32>,
    /// Y distance outside the arena's right border
    pub right_offset: Option<f32>,
}

impl Component for DespawnAtBorderComponent {
    type Storage = DenseVecStorage<Self>;
}

/// Used for despawning entities after a set duration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DespawnTimeLimitComponent {
    /// Amount of time until the entity despawns
    pub duration: f32,
}

impl Component for DespawnTimeLimitComponent {
    type Storage = DenseVecStorage<Self>;
}
