use amethyst::{
    core::math::Vector2,
    ecs::prelude::{Component, DenseVecStorage, NullStorage},
};

use crate::components::SpawnProbabilities;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum EnemyType {
    Pawn,
    Drone,
    Strafer,
    Hauler, //ally
    MissileLauncher,
    Missile,
    RepeaterBody,
    RepeaterHead,
    RepeaterShoulder,
    RepeaterArm,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EnemyComponent {
    pub name: String,
    pub defense_damage: f32,
    #[serde(default = "des_collision_damage")]
    pub collision_damage: f32,
    #[serde(default = "des_poison")]
    pub poison: f32,
    #[serde(default = "des_allied")]
    pub allied: bool,
    pub collectables_probs: SpawnProbabilities,
    pub enemy_type: EnemyType,
    #[serde(default = "des_explosion_sprite_idx")]
    pub explosion_sprite_idx: usize,
    pub target_position: Option<Vector2<f32>>,
}

fn des_explosion_sprite_idx() -> usize {
    0
}
fn des_collision_damage() -> f32 {
    30.0
}
fn des_poison() -> f32 {
    0.0
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
