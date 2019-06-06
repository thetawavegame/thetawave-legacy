use amethyst::ecs::prelude::{Component, DenseVecStorage};


pub struct Background {

}

impl Component for Background {
    type Storage = DenseVecStorage<Self>;
}