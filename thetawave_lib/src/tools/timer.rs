use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Counts down a set amount of time then resets
pub struct Timer {
    period: f32,
    countdown: f32,
}

impl Timer {
    /// Creates a new instance of Timer
    /// Sets both the period and countdown to given value
    pub fn new(period: f32) -> Self {
        Timer {
            period,
            countdown: period,
        }
    }

    /// Updates the timer
    /// Subtracts value from countdown
    /// Resets if countdown reaches 0 and returns true
    pub fn update(&mut self, delta_time: f32) -> bool {
        self.countdown -= delta_time;

        if self.countdown <= 0.0 {
            self.reset();
            return true;
        }
        false
    }

    /// Resets the countdown value to the period
    pub fn reset(&mut self) {
        self.countdown = self.period;
    }

    /// Returns the period of the timer
    pub fn get_period(&self) -> f32 {
        self.period
    }

    pub fn set_period(&mut self, period: f32) {
        self.period = period;
    }

    /// Returns the difference between the period and countdown
    pub fn get_time_left(&self) -> f32 {
        self.period - self.countdown
    }
}
