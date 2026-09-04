use macroquad::prelude::info;
use macroquad::time;

use crate::scene::arena::{playback_speed::PlaybackSpeed, timer::Timer};

/// `PlaybackManager` is responsible for automatically advancing the game state based on its current `speed`.
pub struct PlaybackManager {
    speed: PlaybackSpeed,
    timer: Timer,
    game_started_at: Option<f64>, // Temporary timestamp to check how long a game runs at Turbo speed until it ends after final turn.
}

impl Default for PlaybackManager {
    fn default() -> Self {
        let speed = PlaybackSpeed::default();
        let timer = Timer::new(speed.as_period_in_seconds());

        Self {
            speed,
            timer,
            game_started_at: None,
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
        self.timer.stop();

        if let Some(started_at) = self.game_started_at {
            let elapsed_seconds = time::get_time() - started_at;
            info!("Game ended after {:.2} seconds.", elapsed_seconds);
        }

        self.game_started_at = None;
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
        self.game_started_at = Some(time::get_time());
        self.set_speed(PlaybackSpeed::Turbo);
        self.play();
    }
}
