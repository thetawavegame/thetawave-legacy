use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};

use crate::entities::ConsumableType;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConsumableComponent {
    pub sound_effect: String,
    pub price: usize,
    pub consumable_type: ConsumableType,
}

impl Component for ConsumableComponent {
    type Storage = DenseVecStorage<Self>;
}
