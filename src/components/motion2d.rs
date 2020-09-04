use amethyst::{
  ecs::prelude::{Component, DenseVecStorage},
  core::{math::Vector2}
};

#[derive(Clone)]
pub struct Motion2DComponent {
  pub velocity: Vector2<f32>,
  pub acceleration: Vector2<f32>,
  pub deceleration: Vector2<f32>,
  pub angular_velocity: f32,
  pub angular_acceleration: f32,
  pub angular_deceleration: f32,
}

impl Component for Motion2DComponent {
    type Storage = DenseVecStorage<Self>;
}
