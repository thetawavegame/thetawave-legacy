use amethyst::{
    core::Transform,
    ecs::prelude::{Component, DenseVecStorage, NullStorage},
};

use crate::{
    components::{Fires, Rigidbody, SpawnProbabilities, Spawnable},
    constants::{ARENA_MAX_X, ARENA_MIN_X, ENEMY_BLAST_SPRITE_INDEX},
};

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub enum EnemyType {
    Pawn,
    Drone,
    Strafer,
    Hauler, //ally
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Enemy {
    pub name: String,
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
    #[serde(default = "des_current_velocity_x")]
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
    #[serde(default = "des_crit_chance")]
    pub crit_chance: f32,
    #[serde(default = "des_poison_chance")]
    pub poison_chance: f32,
    #[serde(default = "des_blast_count")]
    pub blast_count: usize,
    #[serde(default = "des_blast_sprite_indicies")]
    pub blast_sprite_indicies: HashMap<String, usize>,
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
fn des_width() -> f32 {
    18.0
}
fn des_height() -> f32 {
    18.0
}
fn des_hitbox_width() -> f32 {
    14.0
}
fn des_hitbox_height() -> f32 {
    14.0
}
fn des_current_velocity_x() -> f32 {
    0.0
}
fn des_acceleration_x() -> f32 {
    2.0
}
fn des_acceleration_y() -> f32 {
    4.0
}
fn des_deceleration_x() -> f32 {
    1.0
}
fn des_deceleration_y() -> f32 {
    1.0
}
fn des_knockback_max_speed() -> f32 {
    100.0
}
fn des_collision_damage() -> f32 {
    30.0
}
fn des_poison() -> f32 {
    0.0
}
fn des_crit_chance() -> f32 {
    0.0
}
fn des_poison_chance() -> f32 {
    0.0
}
fn des_blast_count() -> usize {
    1
}
fn des_blast_sprite_indicies() -> HashMap<String, usize> {
    let mut blast_sprite_indicies = HashMap::new();
    blast_sprite_indicies.insert("normal".to_string(), ENEMY_BLAST_SPRITE_INDEX);
    return blast_sprite_indicies;
}
fn des_allied() -> bool {
    false
}

impl Rigidbody for Enemy {
    fn current_velocity_x(&self) -> f32 {
        self.current_velocity_x
    }

    fn current_velocity_y(&self) -> f32 {
        self.current_velocity_y
    }

    fn acceleration_x(&self) -> f32 {
        self.acceleration_x
    }
    fn acceleration_y(&self) -> f32 {
        self.acceleration_y
    }
    fn deceleration_x(&self) -> f32 {
        self.deceleration_x
    }
    fn deceleration_y(&self) -> f32 {
        self.deceleration_y
    }
    fn max_speed(&self) -> f32 {
        self.max_speed
    }
    fn knockback_max_speed(&self) -> f32 {
        self.knockback_max_speed
    }

    fn set_current_velocity_y(&mut self, value: f32) {
        self.current_velocity_y = value;
    }
    fn set_current_velocity_x(&mut self, value: f32) {
        self.current_velocity_x = value;
    }

    fn constrain_to_arena(&mut self, transform: &mut Transform) {
        let enemy_x = transform.translation().x;
        if (enemy_x - (self.width / 2.0)) < ARENA_MIN_X
            || (enemy_x + (self.width / 2.0)) > ARENA_MAX_X
        {
            self.current_velocity_x = (-1.0) * self.current_velocity_x;
            self.acceleration_x = (-1.0) * self.acceleration_x();
        }
    }
}

impl Fires for Enemy {
    fn blast_sprite_indicies(&self) -> HashMap<String, usize> {
        self.blast_sprite_indicies.clone()
    }
    fn blast_damage(&self) -> f32 {
        self.blast_damage
    }
    fn crit_chance(&self) -> f32 {
        self.crit_chance
    }
    fn poison_chance(&self) -> f32 {
        self.poison_chance
    }
    fn blast_speed(&self) -> f32 {
        self.blast_speed
    }
    fn velocity_x(&self) -> f32 {
        self.current_velocity_x
    }
    fn velocity_y(&self) -> f32 {
        self.current_velocity_y
    }
    fn allied(&self) -> bool {
        self.allied
    }
    fn blast_count(&self) -> usize {
        self.blast_count
    }

    fn fire_reset_timer(&self) -> f32 {
        self.fire_reset_timer
    }
    fn fire_speed(&self) -> f32 {
        self.fire_speed
    }
    fn set_fire_reset_timer(&mut self, value: f32) {
        self.fire_reset_timer = value;
    }
}

impl Spawnable for Enemy {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn init(&mut self) {
        let mut rng = rand::thread_rng();
        let rand_num = rng.gen_range(0, 2);

        if rand_num == 1 {
            self.current_velocity_x = (-1.0) * self.current_velocity_x;
            self.acceleration_x = (-1.0) * self.acceleration_x();
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
