use amethyst::ecs::prelude::{Component, DenseVecStorage};

use std::{
    collections::{HashMap},
    vec::Vec,
};


pub struct EnemyPool {
    available_enemies: Vec<String>,
    enemies: HashMap<String, Enemy>,
}

pub struct EnemySpawner {
    pub spawn_timer: f32,
    pub spawn_interval: f32,
    pub enemies_spawned: u32,
}

impl Component for EnemySpawner {
    type Storage = DenseVecStorage<Self>;
}

pub struct Enemy {
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub fire_speed: f32,
    pub health: f32,
    pub hitbox_width: f32,
    pub hitbox_height: f32,
    pub barrel_damaged: bool,
}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}