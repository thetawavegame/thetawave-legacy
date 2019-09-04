use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::space_shooter::{ARENA_MAX_X, ARENA_MIN_X, ARENA_SPAWN_OFFSET};
use rand::{thread_rng, Rng};

pub type SpawnProbabilities = Vec<(String, f32)>;

pub struct Spawner {
    probabilities: SpawnProbabilities,
    interval: f32,
    counter: u32,
    timer: f32,
    prob_space: f32,
}

impl Component for Spawner {
    type Storage = DenseVecStorage<Self>;
}

impl Spawner {
    /// create spawner instance
    /// ## Parameters
    /// * `probabilities` vector of names with probabilities, sum of probabilities doesn't need to be equal to 1.0,
    /// * `interval` between spawns, it is updated when calling `can_spawn` function
    /// * `counter` total number of available spawns
    pub fn new(probabilities: SpawnProbabilities, interval: f32, counter: u32) -> Self {
        let prob_space = calculate_total_probabilities(&probabilities);
        assert!(prob_space > 0.0);
        Self {
            probabilities,
            interval,
            counter,
            timer: interval,
            prob_space,
        }
    }

    /// spawn random item with position, if timer has expired
    pub fn spawn_with_position(&mut self, dt: f32) -> Option<(f32, &String)> {
        if self.timer > 0.0 || self.counter == 0 {
            self.timer -= dt;
            None
        } else {
            self.timer += self.interval;
            self.counter -= 1;
            Some((
                choose_position(),
                choose_name_precalculated(self.prob_space, &self.probabilities),
            ))
        }
    }
    /// disable specified item from spawning, at the same time increases all other items spawn rate
    /// if all items are disabled, `spawn_with_position` will always return None
    pub fn disable_item(&mut self, item: &String) {
        match self.probabilities.iter_mut().find(|(name, _)| name == item) {
            Some((_, prob)) => {
                *prob = 0.0;
                self.prob_space = calculate_total_probabilities(&self.probabilities);
                if self.prob_space == 0.0 {
                    self.counter = 0;
                }
            }
            _ => {}
        }
    }
}

pub fn choose_random_name(probs: &SpawnProbabilities) -> &String {
    choose_name_precalculated(calculate_total_probabilities(&probs), &probs)
}

fn choose_position() -> f32 {
    let max_width = ARENA_MAX_X - ARENA_SPAWN_OFFSET;
    let min_width = ARENA_MIN_X + ARENA_SPAWN_OFFSET;
    ARENA_MIN_X + ARENA_SPAWN_OFFSET + thread_rng().gen::<f32>() * (max_width - min_width)
}

fn calculate_total_probabilities(probs: &SpawnProbabilities) -> f32 {
    probs.iter().fold(0.0, |sum, item| sum + item.1)
}

fn choose_name_precalculated(total_probs: f32, probs: &SpawnProbabilities) -> &String {
    // pos is in [0..total_probs)
    let pos = thread_rng().gen::<f32>() * total_probs;
    let mut sum = 0.0;
    for (name, value) in probs {
        sum += value;
        if sum > pos {
            return name;
        }
    }
    probs
        .last()
        .map(|(name, _)| name)
        .expect("invalid probabilities, cannot choose name")
}
