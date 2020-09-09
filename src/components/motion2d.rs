use amethyst::{
    core::math::Vector2,
    ecs::prelude::{Component, DenseVecStorage},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Motion2DComponent {
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub deceleration: Vector2<f32>,
    // Max x/y speed an entity can accelerate up to.
    pub max_speed: Vector2<f32>,
    // Max x/y knockback speed of an entity.
    pub knockback_max_speed: Vector2<f32>,
    pub angular_velocity: f32,
    pub angular_acceleration: f32,
    pub angular_deceleration: f32,
}

impl Component for Motion2DComponent {
    type Storage = DenseVecStorage<Self>;
}
