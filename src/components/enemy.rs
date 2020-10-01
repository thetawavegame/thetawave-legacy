use amethyst::{
    core::Transform,
    ecs::prelude::{Component, DenseVecStorage, NullStorage},
};

use crate::{
    components::{Hitbox2DComponent, Motion2DComponent, Rigidbody, SpawnProbabilities},
    constants::{ARENA_MAX_X, ARENA_MIN_X},
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum EnemyType {
    Pawn,
    Drone,
    Strafer,
    Hauler, //ally
    RepeaterBody,
    RepeaterHead,
    RepeaterShoulder,
    RepeaterArm,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Enemy {
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

impl Rigidbody for Enemy {
    fn constrain_to_arena(
        &mut self,
        transform: &mut Transform,
        motion_2d: &mut Motion2DComponent,
        hitbox_2d: &Hitbox2DComponent,
    ) {
        let enemy_x = transform.translation().x;
        if (enemy_x - (hitbox_2d.width / 2.0)) < ARENA_MIN_X
            || (enemy_x + (hitbox_2d.width / 2.0)) > ARENA_MAX_X
        {
            motion_2d.velocity.x *= -1.0;
            motion_2d.acceleration.x *= -1.0;
        }
    }
}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct EnemySpawnerTag;

impl Component for EnemySpawnerTag {
    type Storage = NullStorage<Self>;
}
