use amethyst::{
    core::math::Vector2,
    ecs::prelude::{Component, DenseVecStorage},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutoBlasterComponent {
    pub count: usize,
    pub allied: bool,
    pub shot_velocity: Vector2<f32>,
    pub velocity_multiplier: f32, // what percentage of the velocity from the source motion2d component will be added to the spawned blasts
    pub offset: Vector2<f32>,     // spawn position of blasts offset from center of entity
    pub damage: f32,
    pub poison_damage: f32, // applies damage to blast when rolled
    pub poison_chance: f32,
    pub crit_chance: f32,
    pub size_multiplier: f32,
    pub spacing: f32, // space between blasts when multiple are fired (along x axis)
    pub fire_period: f32, // time between firing blasts
    pub fire_timer: f32, // tracks time between firing blasts
}

impl Component for AutoBlasterComponent {
    type Storage = DenseVecStorage<Self>;
}
