use amethyst::ecs::prelude::{Component, DenseVecStorage};


#[derive(Clone)]
pub struct HealthUnit {

}

impl Component for HealthUnit {
    type Storage = DenseVecStorage<Self>;
}