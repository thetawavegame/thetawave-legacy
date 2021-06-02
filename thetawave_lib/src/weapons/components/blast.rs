use crate::weapons::BlastType;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

/// Core data for blast entities
///
/// Used for data unique to only blast entities
#[derive(Clone)]
pub struct BlastComponent {
    /// Damage that the blast will deal on hit
    pub damage: f32,
    /// Poison damage that blast will apply on hit
    pub poison_damage: f32,
    /// Type of blast
    pub blast_type: BlastType,
}

impl Component for BlastComponent {
    type Storage = DenseVecStorage<Self>;
}
