use crate::entities::ItemType;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ItemComponent {
    pub price: usize,
    pub item_type: ItemType,
}

impl Component for ItemComponent {
    type Storage = DenseVecStorage<Self>;
}
