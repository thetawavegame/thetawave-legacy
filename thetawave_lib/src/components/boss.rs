use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};

pub struct RepeaterComponent {
    pub body: Entity,
    pub head: Entity,
    pub right_shoulder: Entity,
    pub left_shoulder: Entity,
}

impl Component for RepeaterComponent {
    type Storage = DenseVecStorage<Self>;
}
