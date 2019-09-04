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
    Hauler, //actually an ally
}

impl Default for EnemyType {
    fn default() -> Self {
        EnemyType::Pawn
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Enemy {
    #[serde(default)]
    pub width: f32,
    #[serde(default)]
    pub height: f32,
    #[serde(default)]
    pub hitbox_width: f32,
    #[serde(default)]
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
    #[serde(default)]
    pub acceleration_x: f32,
    #[serde(default)]
    pub acceleration_y: f32,
    #[serde(default)]
    pub deceleration_x: f32,
    #[serde(default)]
    pub deceleration_y: f32,
    #[serde(default)]
    pub knockback_max_speed: f32,
    #[serde(default)]
    pub collision_damage: f32,
    pub collectables_probs: SpawnProbabilities,
    pub enemy_type: EnemyType,
}

impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            width: 18.0,
            height: 18.0,
            hitbox_width: 14.0,
            hitbox_height: 14.0,
            health: 220.0,
            sprite_index: 15,
            fires: false,
            fire_speed: 3.0,
            fire_reset_timer: 1.0,
            blast_speed: -60.0,
            blast_damage: 30.0,
            defense_damage: -100.0,
            max_speed: 25.0,
            current_velocity_x: 0.0,
            current_velocity_y: -20.0,
            acceleration_x: 2.0,
            acceleration_y: 4.0,
            deceleration_x: 1.0,
            deceleration_y: 1.0,
            knockback_max_speed: 100.0,
            collision_damage: 30.0,
            collectables_probs: Default::default(),
            enemy_type: Default::default(),
        }
    }
}

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
        }
    }
}

#[derive(Default)]
pub struct EnemySpawnerTag;

impl Component for EnemySpawnerTag {
    type Storage = NullStorage<Self>;
}
