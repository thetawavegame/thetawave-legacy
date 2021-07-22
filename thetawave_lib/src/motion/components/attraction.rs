use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Categories of attracted entities
#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum AttractorCategory {
    Consumable,
    Item,
    Effect,
    Mob,
    Blast,
}

/// Data on how an entity category should be attracted
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AttractData {
    pub radius: f32,
    pub acceleration: f32,
    pub should_repel: bool,
    pub is_active: bool,
}

/// Manages attraction of entities
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AttractorComponent {
    pub attracted_spawnables: HashMap<AttractorCategory, AttractData>,
}

impl Component for AttractorComponent {
    type Storage = DenseVecStorage<Self>;
}
