use amethyst::ecs::prelude::{Component, DenseVecStorage};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CharacterComponent {
    pub money: usize,
    pub collision_damage: f32,
}

impl Component for CharacterComponent {
    type Storage = DenseVecStorage<Self>;
}
