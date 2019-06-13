use amethyst::ecs::prelude::{Component, DenseVecStorage};

use std::{
    collections::{HashMap},
    vec::Vec,
};


#[derive(Clone)]
pub struct Pool<T>{
    pub spawn_list: Vec<String>,
    pub spawns: HashMap<String, T>,
}


pub struct Spawner<T> {
    pub pool: Pool<T>,
    pub spawn_timer: f32,
    pub spawn_interval: f32,
    pub spawn_counter: u32,
}

impl<'a, T> Component for Spawner<T> where T: Send + Sync + 'static{
    type Storage = DenseVecStorage<Self>;
}
