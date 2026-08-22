#[derive(Clone, Copy, Eq, PartialEq)]
pub enum WarriorSeparationStrategy {
    Equal,
    Random,
}

impl WarriorSeparationStrategy {
    pub fn list_all_values() -> Vec<Self> {
        vec![Self::Equal, Self::Random]
    }

    #[inline]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Equal => "Equal",
            Self::Random => "Random",
        }
    }
}
