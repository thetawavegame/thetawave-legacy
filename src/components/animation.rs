use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Animation {
    pub start_idx: usize,
    pub frame_count: usize,
    pub current_frame: usize,
    pub frame_time: f32,
    pub elapsed_time: f32,
    pub forward: bool,
}

impl Component for Animation {
    type Storage = DenseVecStorage<Self>;
}