use std::time::Instant;

use crate::scene::arena::{playback_speed::PlaybackSpeed, timer::Timer};

pub struct PlaybackManager {
    pub speed: PlaybackSpeed,
    timer: Timer,
    game_start: Instant, // Temporary timestamp to check how long a game runs at Turbo speed until it is stopped after final turn.
}

impl Default for PlaybackManager {
    fn default() -> Self {
        let speed = PlaybackSpeed::default();
        let timer = Timer::new(speed.as_period_in_seconds());

        Self {
            speed,
            timer,
            game_start: Instant::now(),
        }
    }
}

impl PlaybackManager {
    #[inline]
    pub const fn is_playing(&self) -> bool {
        self.timer.is_active()
    }

    #[inline]
    pub fn poll(&mut self) -> bool {
        self.timer.poll()
    }

    pub const fn set_speed(&mut self, speed: PlaybackSpeed) {
        self.speed = speed;
        self.timer.set_period(speed.as_period_in_seconds());
    }

    #[inline]
    pub fn play(&mut self) {
        self.timer.start();
    }

    #[inline]
    pub fn stop(&mut self) {
        println!(
            "Game stopped after {} seconds",
            self.game_start.elapsed().as_secs_f32()
        );
        self.timer.stop();
    }

    #[inline]
    pub const fn get_speed(&self) -> PlaybackSpeed {
        self.speed
    }

    pub const fn get_next_speed(&self) -> PlaybackSpeed {
        use PlaybackSpeed::{FastForward2X, FastForward4X, FastForward8X, Normal, Turbo};

        match self.speed {
            Normal => FastForward2X,
            FastForward2X => FastForward4X,
            FastForward4X => FastForward8X,
            FastForward8X => Turbo,
            Turbo => Normal,
        }
    }

    pub fn play_turbo(&mut self) {
        self.game_start = Instant::now();
        self.set_speed(PlaybackSpeed::Turbo);
        self.play();
    }
}
