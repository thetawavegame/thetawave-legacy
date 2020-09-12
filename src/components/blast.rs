use amethyst::ecs::prelude::{Component, DenseVecStorage};

// used for setting sprite, status rolls, and the entity type spawning the blast
#[derive(Clone, Debug)]
pub enum BlastType {
    Ally,
    Enemy,
    AllyPoison,
    AllyCritical,
}

#[derive(Clone)]
pub struct BlastComponent {
    pub damage: f32,
    pub poison_damage: f32,
    pub blast_type: BlastType,
}

impl Component for BlastComponent {
    type Storage = DenseVecStorage<Self>;
}
