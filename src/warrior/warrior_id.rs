/// `WarriorId` serves as a valid index for containers that hold data for all warriors, and also
/// it serves as a foreign key to refer to a `Warrior` without using a reference.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct WarriorId(pub usize);

impl WarriorId {
    /// Return an iterator for all valid `WarriorId` values.
    pub fn list_all_warrior_ids(num_warriors: usize) -> impl Iterator<Item = Self> {
        (0..num_warriors).map(Self)
    }

    /// Return the 1-based display ID for the 0-based `WarriorId`.
    #[allow(clippy::arithmetic_side_effects, reason = "The number is small.")]
    #[must_use]
    pub const fn as_display_id(&self) -> usize {
        self.0 + 1
    }
}
