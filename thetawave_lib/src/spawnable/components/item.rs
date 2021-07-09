use crate::spawnable::ItemType;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};

/// Used for data unique to item entities
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ItemComponent {
    /// Store price of the item
    pub price: usize,
    /// Type of item
    pub item_type: ItemType,
}

impl Component for ItemComponent {
    type Storage = DenseVecStorage<Self>;
}
