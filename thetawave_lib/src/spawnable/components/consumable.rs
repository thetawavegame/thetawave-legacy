use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};

use crate::spawnable::resources::ConsumableType;

/// Used for data unique to consumable entities
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConsumableComponent {
    /// Name of sound effect played on get
    pub sound_effect: String,
    /// Store price of the consumable
    pub price: usize,
    /// Type of consumable
    pub consumable_type: ConsumableType,
}

impl Component for ConsumableComponent {
    type Storage = DenseVecStorage<Self>;
}
