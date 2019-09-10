use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Clone)]
pub enum PhaseType {
    Invasion,
    Rest,
    Boss,
}

#[derive(Clone)]
pub struct Phase {
    pub phase_type: PhaseType,
    pub length: usize,
}

#[derive(Clone)]
pub struct GameMaster{
    pub phase_map: Vec<Phase>,
    pub phase_idx: usize,
    pub last_phase: usize,
    pub current_tick: usize,
    pub tick_timer: f32,
    pub tick_length: f32,
}

impl Component for GameMaster{
    type Storage = DenseVecStorage<Self>;
}

impl GameMaster {
    pub fn iterate_tick(&mut self, dt: f32) {
        if self.tick_timer > 0.0 {
            self.tick_timer -= dt;
        } else {
            //println!("phase index: {}\tcurrent_tick: {}", self.phase_idx, self.current_tick);
            self.tick_timer = self.tick_length;
            self.current_tick += 1;

            if self.phase_idx < self.last_phase && self.current_tick >= self.phase_map[self.phase_idx].length {
                self.phase_idx += 1;
                self.current_tick = 0;
            }
        }
    }
}