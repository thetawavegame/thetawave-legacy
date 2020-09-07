use amethyst::ecs::prelude::{Component, DenseVecStorage};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoBlasterComponent {
    pub count: usize,
    pub allied: bool,
    pub shot_velocity: [f32; 2],
    pub offset_x: f32,
    pub offset_y: f32,
    pub damage: f32,
    pub poison_chance: f32,
    pub crit_chance: f32,
    pub size: f32,
    pub fire_speed: f32,
    pub fire_reset_timer: f32,
}

impl Component for AutoBlasterComponent {
    type Storage = DenseVecStorage<Self>;
}
