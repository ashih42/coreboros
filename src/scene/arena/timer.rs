#[derive(Default)]
pub struct Timer {
    start_time: f64,
    period_in_seconds: f64,
    active: bool,
}

impl Timer {
    pub fn new(period_in_seconds: f64) -> Self {
        Self {
            period_in_seconds,
            start_time: 0.0,
            active: false,
        }
    }

    pub fn start(&mut self) {
        self.start_time = macroquad::time::get_time();
        self.active = true;
    }

    pub fn stop(&mut self) {
        self.active = false;
    }

    /// Check if the amount of time elapsed since `start_time` has exceeded `period_in_seconds`,
    /// and reset `start_time` if true.
    pub fn poll(&mut self) -> bool {
        if !self.active {
            return false;
        }

        let now = macroquad::time::get_time();

        if now > self.start_time + self.period_in_seconds {
            self.start_time = now;
            return true;
        }

        false
    }

    pub fn set_period(&mut self, period_in_seconds: f64) {
        self.period_in_seconds = period_in_seconds;
    }

    #[inline]
    pub fn is_active(&self) -> bool {
        self.active
    }
}
