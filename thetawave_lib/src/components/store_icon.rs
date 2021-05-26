use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Clone)]
pub struct StoreIconComponent {
    pub inventory_index: usize,
}

impl Component for StoreIconComponent {
    type Storage = DenseVecStorage<Self>;
}
