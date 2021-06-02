use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum AttractorCategory {
    Consumable,
    Item,
    Effect,
    Mob,
    Blast,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AttractData {
    pub radius: f32,
    pub acceleration: f32,
    pub should_repel: bool,
    pub is_active: bool,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AttractorComponent {
    pub attracted_spawnables: HashMap<AttractorCategory, AttractData>,
}

impl Component for AttractorComponent {
    type Storage = DenseVecStorage<Self>;
}
