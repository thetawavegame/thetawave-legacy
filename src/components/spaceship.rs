use amethyst::ecs::prelude::{Component, DenseVecStorage};


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
    pub barrel_damage: f32,
    pub pos_x: f32,
    pub pos_y: f32,
}

impl Component for Spaceship {
    type Storage = DenseVecStorage<Self>;
}