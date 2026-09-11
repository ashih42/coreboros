use crate::warrior::warrior_id::WarriorId;

/// `CellSlotAuthor` indicates which warrior last wrote to a part of a `CoreCell`.
///
/// Note: `CellSlotAuthor` is a space-efficient way to store the same information as `Option<WarriorId>`.
/// `CellSlotAuthor` only uses 1 byte, whereas `Option<WarriorId>` uses 16 bytes.
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
    fn from(maybe_warrior_id: Option<WarriorId>) -> Self {
        maybe_warrior_id.map_or(Self::None, |warrior_id| match warrior_id.as_index() {
            0 => Self::Warrior0,
            1 => Self::Warrior1,
            2 => Self::Warrior2,
            3 => Self::Warrior3,
            4 => Self::Warrior4,
            5 => Self::Warrior5,
            6 => Self::Warrior6,
            7 => Self::Warrior7,

            #[allow(
                clippy::unreachable,
                reason = "The engine guarantees at most 8 warriors."
            )]
            _ => unreachable!(),
        })
    }
}

impl From<CellSlotAuthor> for Option<WarriorId> {
    /// Convert from `CellSlotAuthor` to `Option<WarriorId>`.
    fn from(author: CellSlotAuthor) -> Self {
        match author {
            CellSlotAuthor::None => None,
            CellSlotAuthor::Warrior0 => Some(WarriorId::new(0)),
            CellSlotAuthor::Warrior1 => Some(WarriorId::new(1)),
            CellSlotAuthor::Warrior2 => Some(WarriorId::new(2)),
            CellSlotAuthor::Warrior3 => Some(WarriorId::new(3)),
            CellSlotAuthor::Warrior4 => Some(WarriorId::new(4)),
            CellSlotAuthor::Warrior5 => Some(WarriorId::new(5)),
            CellSlotAuthor::Warrior6 => Some(WarriorId::new(6)),
            CellSlotAuthor::Warrior7 => Some(WarriorId::new(7)),
        }
    }
}
