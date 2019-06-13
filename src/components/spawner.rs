use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
};

use std::{
    collections::{HashMap},
    vec::Vec,
};


use crate::space_shooter::{ARENA_MAX_X, ARENA_MIN_X, ARENA_SPAWN_OFFSET};
use rand::{thread_rng, Rng};


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

impl<'a, T> Component for Spawner<T> where T: Send + Sync + 'static {
    type Storage = DenseVecStorage<Self>;
}

impl<'a, T> Spawner<T> where T: Send + Sync + 'static {

    //updates or resets the spawn timer, returns true if can spawn, false if can't
    pub fn can_spawn(&mut self, dt: f32) -> Option<f32> {
        if self.pool.spawn_list.len() == 0 {
            return None;
        }
        if self.spawn_timer > 0.0 {
            self.spawn_timer -= dt;
            return None;
        } else {
            self.spawn_timer = self.spawn_interval;

            let max_width = ARENA_MAX_X - ARENA_SPAWN_OFFSET;
            let min_width = ARENA_MIN_X + ARENA_SPAWN_OFFSET;
            let new_x = ARENA_MIN_X + ARENA_SPAWN_OFFSET + thread_rng().gen::<f32>()* (max_width - min_width);
            return Some(new_x);
        }
    }
}
