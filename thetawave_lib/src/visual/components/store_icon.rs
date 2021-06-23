use amethyst::ecs::prelude::{Component, DenseVecStorage};

/// Used for managing store icons
#[derive(Clone)]
pub struct StoreIconComponent {
    /// Index of the store slot represented
    pub inventory_index: usize,
}

impl Component for StoreIconComponent {
    type Storage = DenseVecStorage<Self>;
}
