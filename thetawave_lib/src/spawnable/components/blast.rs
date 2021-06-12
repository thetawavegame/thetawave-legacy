use crate::weapons::BlastType;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

/// Used for data unique to blast entities
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
