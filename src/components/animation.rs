use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub enum AnimationType {
    PingPong,
    Forward,
}

pub struct Animation {
    pub start_idx: usize,
    pub frame_count: usize,
    pub current_frame: usize,
    pub frame_time: f32,
    pub elapsed_time: f32,
    pub forward: bool, // current direction of the animation frames
    pub animation_type: AnimationType,
}

impl Component for Animation {
    type Storage = DenseVecStorage<Self>;
}