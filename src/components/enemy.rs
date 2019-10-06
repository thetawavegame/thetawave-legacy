use amethyst::{
    ecs::prelude::{Component, DenseVecStorage, NullStorage},
    core::Transform,
};

use crate::{
    components::{Rigidbody, Fires, SpawnProbabilities},
    space_shooter::{ARENA_MIN_X, ARENA_MAX_X},
};

use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum EnemyType {
    Pawn,
    Drone,
    Strafer,
    Hauler, //ally
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Enemy {
    #[serde(default = "des_width")]
    pub width: f32,
    #[serde(default = "des_height")]
    pub height: f32,
    #[serde(default = "des_hitbox_width")]
    pub hitbox_width: f32,
    #[serde(default = "des_hitbox_height")]
    pub hitbox_height: f32,
    pub health: f32,
    pub sprite_index: usize,
    pub fires: bool,
    pub fire_speed: f32,
    pub fire_reset_timer: f32,
    pub blast_speed: f32,
    pub blast_damage: f32,
    pub defense_damage: f32,
    pub max_speed: f32,
    #[serde(default)]
    pub current_velocity_x: f32,
    pub current_velocity_y: f32,
    #[serde(default = "des_acceleration_x")]
    pub acceleration_x: f32,
    #[serde(default = "des_acceleration_y")]
    pub acceleration_y: f32,
    #[serde(default = "des_deceleration_x")]
    pub deceleration_x: f32,
    #[serde(default = "des_deceleration_y")]
    pub deceleration_y: f32,
    #[serde(default = "des_knockback_max_speed")]
    pub knockback_max_speed: f32,
    #[serde(default = "des_collision_damage")]
    pub collision_damage: f32,
    #[serde(default = "des_poison")]
    pub poison: f32,
    pub collectables_probs: SpawnProbabilities,
    pub enemy_type: EnemyType,
}

fn des_width() -> f32 { 18.0 }
fn des_height() -> f32 { 18.0 }
fn des_hitbox_width() -> f32 { 14.0 }
fn des_hitbox_height() -> f32 { 14.0 }
fn des_acceleration_x() -> f32 { 2.0 }
fn des_acceleration_y() -> f32 { 4.0 }
fn des_deceleration_x() -> f32 { 1.0 }
fn des_deceleration_y() -> f32 { 1.0 }
fn des_knockback_max_speed() -> f32 { 100.0 }
fn des_collision_damage() -> f32 { 30.0 }
fn des_poison() -> f32 { 0.0 }

impl Rigidbody for Enemy{
    fn current_velocity_x(&self) ->  f32 {
        self.current_velocity_x
    }

    fn current_velocity_y(&self) -> f32 {
        self.current_velocity_y
    }

    fn acceleration_x(&self) -> f32 { self.acceleration_x }
    fn acceleration_y(&self) -> f32 { self.acceleration_y }
    fn deceleration_x(&self) -> f32 { self.deceleration_x }
    fn deceleration_y(&self) -> f32 { self.deceleration_y }
    fn max_speed(&self) -> f32 { self.max_speed }
    fn knockback_max_speed(&self) -> f32 { self.knockback_max_speed }

    fn set_current_velocity_y(&mut self, value: f32) {
        self.current_velocity_y = value;
    }

    fn set_current_velocity_x(&mut self, value: f32) { self.current_velocity_x = value; }
}

impl Fires for Enemy {
    fn fire_reset_timer(&self) -> f32 { self.fire_reset_timer }
    fn fire_speed(&self) -> f32 { self.fire_speed }
    fn set_fire_reset_timer(&mut self, value: f32) { self.fire_reset_timer = value; }
}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}

impl Enemy {
    pub fn constrain_to_arena(&mut self, transform: &mut Transform) {
        let enemy_x = transform.translation().x;
        if (enemy_x - (self.width/2.0)) < ARENA_MIN_X || (enemy_x + (self.width/2.0)) > ARENA_MAX_X {
            self.current_velocity_x = (-1.0) * self.current_velocity_x;
            self.acceleration_x = (-1.0) * self.acceleration_x();
        }
    }
}

#[derive(Default)]
pub struct EnemySpawnerTag;

impl Component for EnemySpawnerTag {
    type Storage = NullStorage<Self>;
}
