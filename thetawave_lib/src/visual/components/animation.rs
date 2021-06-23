use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};

/// Determines order of animation frames
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum AnimationType {
    PingPong,
    Forward,
    NoAnimation,
}

/// Used for managing sprite animations
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AnimationComponent {
    /// Index of the starting frame
    pub start_idx: usize,
    /// Total number of frames
    pub frame_count: usize,
    /// Current frame
    pub current_frame: usize,
    /// Amount of time per frame
    pub frame_time: f32,
    /// Amount of time passed for the frame
    pub elapsed_time: f32,
    // TODO: remove
    pub forward: bool,
    /// Type of animation
    pub animation_type: AnimationType,
}

impl Component for AnimationComponent {
    type Storage = DenseVecStorage<Self>;
}
