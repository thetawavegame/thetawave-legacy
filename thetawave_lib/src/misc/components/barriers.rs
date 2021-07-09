use amethyst::{
    core::math::Vector2,
    ecs::prelude::{Component, DenseVecStorage},
};

/// Possible directions to push colliding entities
pub enum PushDirection {
    Left,
    Right,
    Down,
    Up,
}

/// Manages barriers
pub struct BarrierComponent {
    /// Speed of deflected entities
    pub deflection_speed: Vector2<f32>,
    /// Damage dealt to colliding entities
    pub damage: f32,
    /// Weather enemies are allowed to pass
    pub enemies_pass: bool,
    /// Direction to push colliding entities
    pub push_direction: PushDirection,
}

impl Component for BarrierComponent {
    type Storage = DenseVecStorage<Self>;
}
