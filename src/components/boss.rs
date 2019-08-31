use amethyst::ecs::prelude::{Component, DenseVecStorage};
use std::{
    vec::Vec,
};

#[derive(Clone)]
pub enum BossType {
    Repeater,
}

#[derive(Clone)]
pub struct Boss {
    pub boss_part_root: BossPart,
    pub boss_type: BossType,
}

impl Component for Boss {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Debug)]
pub struct BossPart {
    pub width: f32,
    pub height: f32,
    pub hitbox_width: f32,
    pub hitbox_height: f32,
    pub sprite_index: usize,
    pub offset_x: f32,
    pub offset_y: f32,
    pub center_of_rotation_offset_x: f32,
    pub center_of_rotation_offset_y: f32,
    pub angle: f32,
    pub boss_part_next: Vec<Option<BossPart>>,
}

impl Component for BossPart {
    type Storage = DenseVecStorage<Self>;
}