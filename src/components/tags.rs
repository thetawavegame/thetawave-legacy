use amethyst::ecs::prelude::{Component, NullStorage};

#[derive(Default)]
pub struct DefenseTag;

impl Component for DefenseTag {
    type Storage = NullStorage<Self>;
}
