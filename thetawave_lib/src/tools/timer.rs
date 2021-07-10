use serde::{Deserialize, Serialize};

/// Counts down timer and resets
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Timer {
    /// Amount of time between resets
    period: f32,
    /// Remaining time until reset
    countdown: f32,
}

impl Timer {
    /// Create a new instance of `Timer`
    ///
    /// # Parameters
    ///
    /// `period` - An `f32` length of the period
    ///
    /// # Returns
    ///
    /// A new instance of `Timer` with `period` and `countdown` set to the given period
    ///
    /// # Panics
    ///
    /// Panics when given period is less than or equal to zero.
    pub fn new(period: f32) -> Self {
        Self::check_period(period);
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
        Self::check_period(period);
        self.period = period;
        if self.countdown > self.period {
            self.reset()
        }
    }

    /// Returns the difference between the period and countdown
    pub fn get_time_left(&self) -> f32 {
        self.period - self.countdown
    }

    fn check_period(period: f32) {
        if period <= 0.0 {
            panic!("Period of timer must be greater than zero.")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn false_on_update_greater_than_zero() {
        let mut timer = Timer::new(2.0);
        assert!(!timer.update(1.0))
    }

    #[test]
    fn true_on_update_equal_to_zero() {
        let mut timer = Timer::new(1.0);
        assert!(timer.update(1.0))
    }

    #[test]
    fn true_on_update_less_than_zero() {
        let mut timer = Timer::new(0.5);
        assert!(timer.update(1.0))
    }

    #[test]
    fn zero_time_left_on_reset() {
        let mut timer = Timer::new(5.0);
        timer.update(1.0);
        timer.reset();
        assert!(timer.get_time_left() == 0.0)
    }

    #[test]
    fn zero_time_left_on_set_period_when_less_than_countdown() {
        let mut timer = Timer::new(5.0);
        timer.update(1.0);
        timer.set_period(3.0);
        assert!(timer.get_time_left() == 0.0);
    }

    #[test]
    #[should_panic]
    fn panic_on_new_with_zero_period() {
        Timer::new(0.0);
    }

    #[test]
    #[should_panic]
    fn panic_on_new_with_negative_period() {
        Timer::new(-1.0);
    }

    #[test]
    #[should_panic]
    fn panic_on_set_period_to_zero() {
        let mut timer = Timer::new(1.0);
        timer.set_period(0.0);
    }

    #[test]
    #[should_panic]
    fn panic_on_set_period_to_negative() {
        let mut timer = Timer::new(1.0);
        timer.set_period(-1.0);
    }
}
