use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};

/// Manages repeater boss
pub struct RepeaterComponent {
    /// Body entity of repeater
    pub body: Entity,
    /// Head entity of repeater
    pub head: Entity,
    /// Right shoulder entity of repeater
    pub right_shoulder: Entity,
    /// Left shoulder entity of repeater
    pub left_shoulder: Entity,
}

impl Component for RepeaterComponent {
    type Storage = DenseVecStorage<Self>;
}
