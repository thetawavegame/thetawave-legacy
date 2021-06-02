use amethyst::{
    core::math::Vector2,
    ecs::prelude::{Component, DenseVecStorage},
};

#[derive(Clone)]
pub enum PushDirection {
    Left,
    Right,
    Down,
    Up,
}

#[derive(Clone)]
pub struct BarrierComponent {
    pub deflection_speed: Vector2<f32>,
    pub damage: f32,
    pub enemies_pass: bool,
    pub push_direction: PushDirection,
}

impl Component for BarrierComponent {
    type Storage = DenseVecStorage<Self>;
}
