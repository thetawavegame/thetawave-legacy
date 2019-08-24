use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
    core::Transform,
};

use crate::{
    components::{Pool, Consumable, Rigidbody, Fires},
    space_shooter::{ARENA_MIN_X, ARENA_MAX_X},
};


#[derive(Clone)]
pub enum EnemyType {
    Pawn,
    Drone,
    Hauler, //actually an ally
}

#[derive(Clone)]
pub struct Enemy {
    pub width: f32,
    pub height: f32,
    pub health: f32,
    pub hitbox_width: f32,
    pub hitbox_height: f32,
    pub sprite_index: usize,
    pub fires: bool,
    pub fire_speed: f32,
    pub fire_reset_timer: f32,
    pub blast_speed: f32,
    pub blast_damage: f32,
    pub defense_damage: f32,
    pub max_speed: f32,
    pub current_velocity_x: f32,
    pub current_velocity_y: f32,
    pub acceleration_x: f32,
    pub acceleration_y: f32,
    pub deceleration_x: f32,
    pub deceleration_y: f32,
    pub knockback_max_speed: f32,
    pub collision_damage: f32,
    pub consumable_pool: Pool<Consumable>,
    pub drop_chance: f32,
    pub enemy_type: EnemyType,
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