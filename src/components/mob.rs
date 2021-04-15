use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::entities::{LootTable, SpawnableType};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MobComponent {
    pub defense_damage: f32,
    #[serde(default = "des_collision_damage")]
    pub collision_damage: f32,
    #[serde(default = "des_allied")]
    pub allied: bool,
    pub loot_probs: LootTable,
    pub spawnable_type: SpawnableType,
}

fn des_collision_damage() -> f32 {
    30.0
}
fn des_allied() -> bool {
    false
}

impl Component for MobComponent {
    type Storage = DenseVecStorage<Self>;
}
