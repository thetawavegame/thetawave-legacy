pub struct Timer {
    period: f32,
    countdown: f32,
}

impl Timer {
    pub fn new(period: f32) -> Self {
        Timer {
            period,
            countdown: period,
        }
    }

    pub fn update(&mut self, delta_time: f32) -> bool {
        self.countdown -= delta_time;

        if self.countdown <= 0.0 {
            self.reset();
            return true;
        }
        false
    }

    pub fn reset(&mut self) {
        self.countdown = self.period;
    }

    pub fn get_period(&self) -> f32 {
        self.period
    }

    pub fn get_time_left(&self) -> f32 {
        self.period - self.countdown
    }
}
