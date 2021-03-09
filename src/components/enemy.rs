use amethyst::ecs::prelude::{Component, DenseVecStorage, NullStorage};

use crate::{components::SpawnProbabilities, entities::EntityType};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EnemyComponent {
    pub defense_damage: f32,
    #[serde(default = "des_collision_damage")]
    pub collision_damage: f32,
    #[serde(default = "des_allied")]
    pub allied: bool,
    pub loot_probs: SpawnProbabilities,
    pub entity_type: EntityType,
}

fn des_collision_damage() -> f32 {
    30.0
}
fn des_allied() -> bool {
    false
}

impl Component for EnemyComponent {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct EnemySpawnerTag;

impl Component for EnemySpawnerTag {
    type Storage = NullStorage<Self>;
}
