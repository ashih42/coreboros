/// `WarriorId` serves as an identifier or a tag for a specific `Warrior`, instead of using references.
/// Also, it can be used as an index into containers to find resources for a specific `Warrior`.
///
/// Note: The wrapped value may only be some value within the range [0, 7], because currently the game enforces the limit of at most 8 warriors.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct WarriorId(usize);

impl WarriorId {
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Return an iterator for all valid `WarriorId` values in ascending order.
    pub fn list_all_warrior_ids(num_warriors: usize) -> impl Iterator<Item = Self> {
        (0..num_warriors).map(Self)
    }

    /// Convert `WarriorId` to a 0-based index for array access.
    #[inline]
    #[must_use]
    pub const fn as_index(&self) -> usize {
        self.0
    }

    /// Convert `WarriorId` to a 1-based display number.
    /// Example: WarriorId(0) uses array index 0 for its resources, while it is displayed in the UI as "Warrior 1".
    #[inline]
    #[must_use]
    pub const fn as_display_number(&self) -> usize {
        #[allow(
            clippy::arithmetic_side_effects,
            reason = "Because `self.0` is constrained within the range [0, 7], this expression cannot cause overflow."
        )]
        {
            self.0 + 1
        }
    }
}
