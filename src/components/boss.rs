use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};

pub struct Repeater {
    pub body: Entity,
    pub head: Entity,
    pub right_shoulder: Entity,
    pub left_shoulder: Entity,
}

impl Component for Repeater {
    type Storage = DenseVecStorage<Self>;
}
