use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use std::vec::Vec;


pub struct RollBar {
    pub x_pos: f32,
    pub y_pos: f32,
    pub roll_stack: Vec<Entity>,
}

impl Component for RollBar {
    type Storage = DenseVecStorage<Self>;
}