/// `WarriorSeparationStrategy` indicates how to determine the starting locations in the core
/// for each warrior's instructions.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum WarriorSeparationStrategy {
    Equal,
    Random,
}

impl WarriorSeparationStrategy {
    pub fn list_all_values() -> Box<[Self]> {
        Box::new([Self::Equal, Self::Random])
    }

    #[inline]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Equal => "Equal",
            Self::Random => "Random",
        }
    }
}
