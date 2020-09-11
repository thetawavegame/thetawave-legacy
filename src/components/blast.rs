use amethyst::ecs::prelude::{Component, DenseVecStorage};

// used for setting sprite, status rolls, and which entities it can hit
#[derive(Clone, Debug)]
pub enum BlastType {
    Ally,
    Enemy,
    AllyPoison,
    AllyCritical,
}

#[derive(Clone)]
pub struct Blast {
    pub damage: f32,
    pub poison_damage: f32,
    pub velocity_factor: f32,
    pub blast_type: BlastType,
}

impl Component for Blast {
    type Storage = DenseVecStorage<Self>;
}
