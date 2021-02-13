use amethyst::ecs::prelude::{Component, NullStorage};

#[derive(Default)]
pub struct DefenseTag;

impl Component for DefenseTag {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct ArenaBorderTag;

impl Component for ArenaBorderTag {
    type Storage = NullStorage<Self>;
}
