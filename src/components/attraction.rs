use crate::entities::SpawnableCategory;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AttractData {
    pub radius: f32,
    pub acceleration: f32,
    pub should_repel: bool,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AttractorComponent {
    pub attracted_spawnables: HashMap<SpawnableCategory, AttractData>,
}

impl Component for AttractorComponent {
    type Storage = DenseVecStorage<Self>;
}
