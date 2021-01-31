use amethyst::ecs::prelude::{Component, NullStorage};

#[derive(Default)]
pub struct DefenseTag;

impl Component for DefenseTag {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct DespawnAtBottomTag;

impl Component for DespawnAtBottomTag {
    type Storage = NullStorage<Self>;
}
