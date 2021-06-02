use crate::entities::ItemType;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PlayerComponent {
    pub money: usize,
    pub collision_damage: f32,
    pub items: Vec<ItemType>,
}

impl Component for PlayerComponent {
    type Storage = DenseVecStorage<Self>;
}
