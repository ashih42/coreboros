/// `CoreInitializationStrategy` indicates how to fill the addresses in the core that are
/// not initialized with a warrior's instructions.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum CoreInitializationStrategy {
    FillDat00,
    Leftover,
    Random,
}

impl CoreInitializationStrategy {
    pub fn list_all_values() -> Box<[Self]> {
        Box::new([Self::FillDat00, Self::Leftover, Self::Random])
    }

    #[inline]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::FillDat00 => "Fill with Dat 0, 0",
            Self::Leftover => "Leftover from last game",
            Self::Random => "Random",
        }
    }
}
