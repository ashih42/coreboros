/// `WarriorId` serves as a valid index for containers that hold data for all warriors, and also
/// it serves as a foreign key to refer to a `Warrior` without using a reference.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct WarriorId(usize);

impl WarriorId {
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Return an iterator for all valid `WarriorId` values in order.
    pub fn list_all_warrior_ids(num_warriors: usize) -> impl Iterator<Item = Self> {
        (0..num_warriors).map(Self)
    }

    /// Convert `WarriorId` to a 0-based index for array access.
    #[must_use]
    pub const fn as_index(&self) -> usize {
        self.0
    }

    /// Convert `WarriorId` to a 1-based display number.
    #[must_use]
    pub const fn as_display_number(&self) -> usize {
        #[allow(clippy::arithmetic_side_effects, reason = "The number is small.")]
        {
            self.0 + 1
        }
    }
}
