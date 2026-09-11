use crate::{core_war::core_number::CoreNumber, instruction::addressing_mode::AddressingMode};

/// `CoreOperand` is an operand component of a `CoreInstruction`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CoreOperand {
    pub mode: AddressingMode,
    pub number: CoreNumber,
}

impl CoreOperand {
    #[must_use]
    pub const fn new(mode: AddressingMode, number: CoreNumber) -> Self {
        Self { mode, number }
    }

    pub const fn direct(number: CoreNumber) -> Self {
        Self::new(AddressingMode::Direct, number)
    }
}
