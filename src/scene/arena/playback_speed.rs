use strum::AsRefStr;

#[derive(Debug, Default, Clone, Copy, AsRefStr)]

/// `PlaybackSpeed` is a preset of possible speed values, used by `PlaybackManager`.
pub enum PlaybackSpeed {
    #[default]
    #[strum(serialize = "Normal")]
    Normal,

    #[strum(serialize = "2X")]
    FastForward2X,

    #[strum(serialize = "4X")]
    FastForward4X,

    #[strum(serialize = "8X")]
    FastForward8X,

    #[strum(serialize = "Turbo")]
    Turbo,
}

impl PlaybackSpeed {
    #[inline]
    pub const fn as_period_in_seconds(self) -> f64 {
        match self {
            Self::Normal => 1.0,
            Self::FastForward2X => 0.5,
            Self::FastForward4X => 0.25,
            Self::FastForward8X => 0.125,
            Self::Turbo => 0.0,
        }
    }
}
