use crate::spawnable::ItemType;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};

/// Component for managing core attributes of player
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PlayerComponent {
    /// Amount of money the player has
    pub money: usize,
    /// Amount of collision damage the player deals
    pub collision_damage: f32,
    /// All the items the player has collected
    pub items: Vec<ItemType>,
}

impl Component for PlayerComponent {
    type Storage = DenseVecStorage<Self>;
}
