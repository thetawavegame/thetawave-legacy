use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use std::vec::Vec;
use crate::components::HealthUnit;


pub struct HealthBar {
    pub x_pos: f32,
    pub y_pos: f32,
    pub health_stack: Vec<Entity>,
}

impl Component for HealthBar {
    type Storage = DenseVecStorage<Self>;
}