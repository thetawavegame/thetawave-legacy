use amethyst::ecs::prelude::{Component, DenseVecStorage};


pub struct Spaceship {
    pub width: f32,
    pub height: f32,
    pub current_velocity_x: f32,
    pub current_velocity_y: f32,
    pub max_speed: f32,
    pub acceleration_x: f32,
    pub deceleration_x: f32,
    pub acceleration_y: f32,
    pub deceleration_y: f32,
    pub fire_speed: f32,
    pub fire_reset_timer: f32,
    pub damage: i32,
    pub x_velocity: f32,
    pub y_velocity: f32,
}

impl Component for Spaceship {
    type Storage = DenseVecStorage<Self>;
}