use std::fmt;

use crate::instruction::addressing_mode::AddressingMode;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Operand {
    pub mode: AddressingMode,
    pub number: i32,
}

impl Operand {
    #[must_use]
    pub const fn new(mode: AddressingMode, number: i32) -> Self {
        Self { mode, number }
    }

    // #[must_use]
    // pub const fn immediate_zero() -> Self {
    //     Self::new(AddressingMode::Immediate, 0)
    // }

    // #[must_use]
    // pub const fn direct_zero() -> Self {
    //     Self::new(AddressingMode::Direct, 0)
    // }

    pub const fn immediate(number: i32) -> Self {
        Self::new(AddressingMode::Immediate, number)
    }

    pub const fn direct(number: i32) -> Self {
        Self::new(AddressingMode::Direct, number)
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.mode, self.number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operand_to_string() {
        assert_eq!(
            "#42",
            Operand::new(AddressingMode::Immediate, 42).to_string()
        );
    }
}
