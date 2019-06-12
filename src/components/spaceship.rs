use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
    core::Transform,
};
use crate::components::Rigidbody;




pub struct Spaceship {
    pub width: f32,
    pub height: f32,
    pub hitbox_width: f32,
    pub hitbox_height: f32,
    pub current_velocity_x: f32,
    pub current_velocity_y: f32,
    pub max_speed: f32,
    pub acceleration_x: f32,
    pub deceleration_x: f32,
    pub acceleration_y: f32,
    pub deceleration_y: f32,
    pub fire_speed: f32,
    pub fire_reset_timer: f32,
    pub damage: f32,
    pub barrel_cooldown: f32,
    pub barrel_reset_timer: f32,
    pub barrel_speed: f32,
    pub barrel_action_left: bool,
    pub barrel_action_right: bool,
    pub barrel_duration: f32,
    pub barrel_action_timer: f32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub blast_speed: f32,
    pub max_health: f32,
    pub health: f32,
    pub knockback_max_speed: f32,
    pub steel_barrel: bool,
    pub collision_damage: f32,
}

impl Rigidbody for Spaceship{
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

    fn set_current_velocity_y(&mut self, value: f32) { self.current_velocity_y = value; }
    fn set_current_velocity_x(&mut self, value: f32) { self.current_velocity_x = value; }
}

impl Component for Spaceship {
    type Storage = DenseVecStorage<Self>;
}