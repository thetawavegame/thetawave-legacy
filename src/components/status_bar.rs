use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use std::vec::Vec;


pub enum StatusType {
    Health,
    Defense,
    Roll,
}

pub struct StatusBar {
    pub status_type: StatusType,
    pub x_pos: f32,
    pub y_pos: f32,
    pub status_unit_stack: Vec<Entity>,
    pub unit_limit: f32,
}

impl Component for StatusBar {
    type Storage = DenseVecStorage<Self>;
}