use crate::tools::Timer;
use serde::{Deserialize, Serialize};

/// Pool type of randomly spawned individual spawnables
#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum InvasionRandomPool {
    Level1Easy,
}

/// Pool type of randomly spawned formations of spawnables
#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum InvasionFormationPool {
    Level1Easy,
    Level1Medium,
}

/// Type of phase (building blocks of levels)
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum PhaseType {
    InvasionRandom(InvasionRandomPool),
    InvasionFormation(InvasionFormationPool),
    Rest,
    Boss,
}

/// Type of boss in a phase
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum BossType {
    Repeater,
    None,
}

/// Holds data about a phase
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Phase {
    /// Type of phase
    pub phase_type: PhaseType,
    /// Type of boss in the phase
    pub boss_type: BossType,
    /// Length of the phase (seconds)
    pub length: usize,
    /// Weather the boss has been spawned
    pub boss_spawned: bool,
}

/// Resource for managing phases
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct PhaseManagerResource {
    /// Phases in the level
    pub phase_map: Vec<Phase>,
    /// Current index in `phase_map`
    pub phase_idx: usize,
    /// Current tick in the phase
    pub current_tick: usize,
    /// Timer for counting ticks
    pub phase_timer: Timer,
}

impl PhaseManagerResource {
    /// Get the current phase of the level
    pub fn get_current_phase(&self) -> Option<&Phase> {
        self.phase_map.get(self.phase_idx)
    }

    /// Get the type of the current phase
    pub fn get_current_phase_type(&self) -> Option<&PhaseType> {
        if let Some(phase) = self.get_current_phase() {
            Some(&phase.phase_type)
        } else {
            None
        }
    }

    /// Update the tick and phase
    pub fn update(&mut self, dt: f32) {
        // update tick
        if self.get_current_phase().is_some() {
            if self.phase_timer.update(dt) {
                println!(
                    "phase index: {}\tcurrent_tick: {}",
                    self.phase_idx, self.current_tick
                );
                self.current_tick += 1;
            }
            // check if the phase is over
            if let Some(phase) = self.get_current_phase() {
                if self.current_tick >= phase.length {
                    if self.phase_idx == self.phase_map.len() - 1 {
                        // TODO: end level
                    }
                    self.phase_idx += 1;
                    self.current_tick = 0;
                }
            }
        }
    }
}
