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
    pub boss_spawned: bool,
}

/*
pub struct Phasemap {
    pub phases: Vec<PhaseType>,

}
*/

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