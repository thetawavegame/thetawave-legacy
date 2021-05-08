use crate::entities::SpawnableCategory;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AttractorComponent {
    pub attracted_spawnables: Vec<SpawnableCategory>,
    pub attraction_radius: f32,
    pub attraction_acceleration: f32,
}

impl Component for AttractorComponent {
    type Storage = DenseVecStorage<Self>;
}
