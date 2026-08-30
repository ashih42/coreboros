/// `CoreDimension` indicates how big the core is.
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

    /// Return the number of cells defined in this `CoreDimension`.
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

    /// Return (`width`, `height`) for the grid view of this `CoreDimension`.
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

    /// Return (`num_sectors`, `num_rings`) for the ring view of this `CoreDimension`.
    #[inline]
    pub const fn as_ring_dimensions(self) -> (usize, usize) {
        match self {
            Self::Pico => (10, 2),
            Self::Nano => (20, 4),
            Self::Micro => (40, 8),
            Self::Mini => (80, 16),
            Self::Small => (100, 20),
            Self::Medium => (160, 32),
            Self::Large => (200, 40),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_grid_dimensions() {
        for dimension in CoreDimension::list_all_values() {
            let size = dimension.as_size();
            let (width, height) = dimension.as_grid_dimensions();

            assert_eq!(size, width * height);
        }
    }

    #[test]
    fn test_as_ring_dimensions() {
        for dimension in CoreDimension::list_all_values() {
            let size = dimension.as_size();
            let (num_sectors, num_rings) = dimension.as_ring_dimensions();

            assert_eq!(size, num_sectors * num_rings);
        }
    }
}
