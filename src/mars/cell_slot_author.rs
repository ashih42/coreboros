use crate::warrior::warrior_id::WarriorId;

/// `CellSlotAuthor` indicates which warrior last wrote to this part of a `CoreCell`.
///
/// `CellSlotAuthor` is a space-efficient way to store the same information as `Option<WarriorId>`.
/// Whereas `Option<WarriorId>` uses 16 bytes, `CellSlotAuthor` only uses 1 byte.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum CellSlotAuthor {
    None,
    Warrior0,
    Warrior1,
    Warrior2,
    Warrior3,
    Warrior4,
    Warrior5,
    Warrior6,
    Warrior7,
}

impl CellSlotAuthor {
    /// Check if this `CellSlotAuthor` points to a `Warrior`.
    pub fn is_some(self) -> bool {
        self != Self::None
    }
}

impl From<Option<WarriorId>> for CellSlotAuthor {
    /// Convert from `Option<WarriorId>` to `CellSlotAuthor`.
    fn from(value: Option<WarriorId>) -> Self {
        match value {
            None => Self::None,
            Some(0) => Self::Warrior0,
            Some(1) => Self::Warrior1,
            Some(2) => Self::Warrior2,
            Some(3) => Self::Warrior3,
            Some(4) => Self::Warrior4,
            Some(5) => Self::Warrior5,
            Some(6) => Self::Warrior6,
            Some(7) => Self::Warrior7,

            #[allow(clippy::unreachable, reason = "The game only allows up to 8 warriors.")]
            Some(warrior_id) => unreachable!("Invalid warrior_id: {warrior_id}"),
        }
    }
}

impl From<CellSlotAuthor> for Option<WarriorId> {
    /// Convert from `CellSlotAuthor` to `Option<WarriorId>`.
    fn from(author: CellSlotAuthor) -> Self {
        match author {
            CellSlotAuthor::None => None,
            CellSlotAuthor::Warrior0 => Some(0),
            CellSlotAuthor::Warrior1 => Some(1),
            CellSlotAuthor::Warrior2 => Some(2),
            CellSlotAuthor::Warrior3 => Some(3),
            CellSlotAuthor::Warrior4 => Some(4),
            CellSlotAuthor::Warrior5 => Some(5),
            CellSlotAuthor::Warrior6 => Some(6),
            CellSlotAuthor::Warrior7 => Some(7),
        }
    }
}
