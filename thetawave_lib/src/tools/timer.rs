use float_cmp::approx_eq;
use serde::{Deserialize, Serialize};

/// Units of least precision
const ULPS: i32 = 2;

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
    #[must_use]
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

    /// Returns the time passed since the last reset
    #[must_use]
    pub fn get_time_passed(&self) -> f32 {
        self.period - self.countdown
    }

    fn check_period(period: f32) {
        if period <= 0.0 {
            panic!("Period of timer must be greater than zero.")
        }
    }
}

impl PartialEq for Timer {
    fn eq(&self, other: &Self) -> bool {
        approx_eq!(f32, self.countdown, other.countdown, ulps = ULPS)
            && approx_eq!(f32, self.period, other.period, ulps = ULPS)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_updates_countdown() {
        // Instantiate timer with a period of 3.0
        let mut timer = Timer::new(3.0);
        // Update timer and assert it does not reset
        assert!(!timer.update(1.0));
        // Assert that 1.0 seconds have passed
        assert!(approx_eq!(f32, timer.get_time_passed(), 1.0, ulps = ULPS));
        // Assert timer matches values
        assert_eq!(
            timer,
            Timer {
                period: 3.0,
                countdown: 2.0,
            }
        );
    }

    #[test]
    fn test_update_resets_countdown() {
        // Instantiate timer with a period of 1.0
        let mut timer = Timer::new(1.0);
        // Update timer and assert it resets
        assert!(timer.update(1.0));
        // Assert that 0.0 seconds have passed
        assert!(approx_eq!(f32, timer.get_time_passed(), 0.0, ulps = ULPS));
        // Assert timer matches values
        assert_eq!(
            timer,
            Timer {
                period: 1.0,
                countdown: 1.0,
            }
        );
    }

    #[test]
    fn test_update_resets_countdown_2() {
        // Instantiate timer with a period of 0.5
        let mut timer = Timer::new(0.5);
        // Update timer and assert it resets
        assert!(timer.update(1.0));
        // Assert that 0.0 seconds have passed
        assert!(approx_eq!(f32, timer.get_time_passed(), 0.0, ulps = ULPS));
        // Assert timer matches values
        assert_eq!(
            timer,
            Timer {
                period: 0.5,
                countdown: 0.5,
            }
        );
    }

    #[test]
    fn test_manual_reset() {
        // Instantiate timer with a period of 5.0
        let mut timer = Timer::new(5.0);
        // Update timer
        timer.update(1.0);
        // Manually reset timer
        timer.reset();
        // Assert that 0.0 seconds have passed
        assert!(approx_eq!(f32, timer.get_time_passed(), 0.0, ulps = ULPS));
        // Assert timer matches values
        assert_eq!(
            timer,
            Timer {
                period: 5.0,
                countdown: 5.0,
            }
        );
    }

    #[test]
    fn test_set_period() {
        // Instantiate timer with a period of 5.0
        let mut timer = Timer::new(5.0);
        // Update timer
        timer.update(1.0);
        // Set period to 3.0
        timer.set_period(3.0);
        // Assert that 0.0 seconds have passed
        assert!(approx_eq!(f32, timer.get_time_passed(), 0.0, ulps = ULPS));
        // Assert timer matches values
        assert_eq!(
            timer,
            Timer {
                period: 3.0,
                countdown: 3.0,
            }
        );
    }

    #[test]
    fn test_set_period_2() {
        // Instantiate timer with a period of 5.0
        let mut timer = Timer::new(5.0);
        // Update timer
        timer.update(1.0);
        // Set period to 6.0
        timer.set_period(6.0);
        // Assert that 2.0 seconds have passed
        assert!(approx_eq!(f32, timer.get_time_passed(), 2.0, ulps = ULPS));
        // Assert timer matches values
        assert_eq!(
            timer,
            Timer {
                period: 6.0,
                countdown: 4.0,
            }
        );
    }

    #[test]
    #[should_panic]
    fn test_zero_period() {
        // Instantiate timer with a period of 0.0
        Timer::new(0.0);
    }

    #[test]
    #[should_panic]
    fn test_negative_period() {
        // Instantiate timer with a period of -1.0
        Timer::new(-1.0);
    }

    #[test]
    #[should_panic]
    fn test_set_period_zero() {
        // Instantiate timer with a period of 1.0
        let mut timer = Timer::new(1.0);
        // Set period to 0.0
        timer.set_period(0.0);
    }

    #[test]
    #[should_panic]
    fn test_set_period_negative() {
        // Instantiate timer with a period of 1.0
        let mut timer = Timer::new(1.0);
        // Set period -1.0
        timer.set_period(-1.0);
    }
}
