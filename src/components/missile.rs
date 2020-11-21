use amethyst::ecs::prelude::{Component, DenseVecStorage};

use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct MissileComponent {
    pub damage: f32,
}

impl Component for BlastComponent {
    type Storage = DenseVecStorage<Self>;
}
