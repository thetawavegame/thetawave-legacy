use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use std::vec::Vec;


pub struct DefenseBar {
    pub x_pos: f32,
    pub y_pos: f32,
    pub defense_stack: Vec<Entity>,
    pub max_defense: f32,
    pub defense: f32,
}

impl Component for DefenseBar {
    type Storage = DenseVecStorage<Self>;
}