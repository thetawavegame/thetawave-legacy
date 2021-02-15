use amethyst::{
    core::math::Vector2,
    ecs::prelude::{Component, DenseVecStorage},
};

use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct BarrierComponent {
    pub deflection_velocity: Vector2<f32>,
    pub damage: f32,
}

impl Component for BarrierComponent {
    type Storage = DenseVecStorage<Self>;
}
