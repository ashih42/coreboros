#[derive(Clone, Copy, Eq, PartialEq)]
pub enum CoreDimension {
    Pico,
    Nano,
    Micro,
    Mini,
    Small,
    Medium,
    Large,
}

impl CoreDimension {
    pub fn list_all_values() -> Box<[Self]> {
        Box::new([
            Self::Pico,
            Self::Nano,
            Self::Micro,
            Self::Mini,
            Self::Small,
            Self::Medium,
            Self::Large,
        ])
    }

    #[inline]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Pico => "Pico (5 x 4 = 20)",
            Self::Nano => "Nano (10 x 8 = 80)",
            Self::Micro => "Micro  (20 x 16 = 320)",
            Self::Mini => "Mini (40 x 32 = 1280)",
            Self::Small => "Small (50 x 40 = 2000)",
            Self::Medium => "Medium (80 x 64 = 5120)",
            Self::Large => "Large (100 x 80 = 8000)",
        }
    }

    #[inline]
    pub const fn as_size(self) -> usize {
        match self {
            Self::Pico => 20,
            Self::Nano => 80,
            Self::Micro => 320,
            Self::Mini => 1_280,
            Self::Small => 2_000,
            Self::Medium => 5_120,
            Self::Large => 8_000,
        }
    }

    #[inline]
    pub const fn as_grid_dimensions(self) -> (usize, usize) {
        match self {
            Self::Pico => (5, 4),
            Self::Nano => (10, 8),
            Self::Micro => (20, 16),
            Self::Mini => (40, 32),
            Self::Small => (50, 40),
            Self::Medium => (80, 64),
            Self::Large => (100, 80),
        }
    }
}
