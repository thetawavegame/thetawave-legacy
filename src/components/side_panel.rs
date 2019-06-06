use amethyst::ecs::prelude::{Component, DenseVecStorage};


#[derive(Clone)]
pub struct SidePanel {

}

impl Component for SidePanel {
    type Storage = DenseVecStorage<Self>;
}
