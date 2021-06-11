use crate::{entities::MobType, resources::DropRolls};
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};

/// Used for data unique to mob entities
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MobComponent {
    /// Damage to defense upon crossing bottom arena border
    pub defense_damage: f32,
    /// Damage dealt to colliding entities
    pub collision_damage: f32,
    /// Possible drops
    pub drop_rolls: DropRolls,
    // Type of mob
    pub mob_type: MobType,
}

impl Component for MobComponent {
    type Storage = DenseVecStorage<Self>;
}
