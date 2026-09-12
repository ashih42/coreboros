use std::fmt;

use crate::instruction::{operand::Operand, operation::Operation};

pub mod addressing_mode;
pub mod modifier;
pub mod opcode;
pub mod operand;
pub mod operation;

/// `Instruction` is a single assembly instruction in a successfully compiled `Warrior` program,
/// and it contains arbitrary `i32` operand numbers, as a temporary, generalized form that can be
/// loaded to `Core` with any `core_size`.
///
/// Thus, this is an instruction that has not been loaded to the `Core`, and it cannot be executed.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    pub operation: Operation,
    pub a: Operand,
    pub b: Operand,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}, {}", self.operation, self.a, self.b)
    }
}

impl Instruction {
    #[must_use]
    pub const fn new(operation: Operation, a: Operand, b: Operand) -> Self {
        Self { operation, a, b }
    }

    /// Convert this single instruction to Load File format.
    /// Reference: <https://corewar.co.uk/standards/icws94.htm#3.0>
    #[must_use]
    pub fn as_load_file(&self) -> String {
        const OPERATION_WIDTH: usize = 11;
        const OPERAND_WIDTH: usize = 12;

        format!(
            "{opcode}.{modifier:<OPERATION_WIDTH$}{a_mode}{a_number:<OPERAND_WIDTH$}{b_mode}{b_number}",
            opcode = self.operation.opcode,
            modifier = self.operation.modifier,
            a_mode = self.a.mode,
            a_number = self.a.number,
            b_mode = self.b.mode,
            b_number = self.b.number,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::{addressing_mode::AddressingMode, modifier::Modifier, opcode::Opcode};

    #[test]
    fn inspect_sizes() {
        println!("Opcode: {}", std::mem::size_of::<Opcode>());
        println!("Modifier: {}", std::mem::size_of::<Modifier>());
        println!("AddressingMode: {}", std::mem::size_of::<AddressingMode>());

        println!("Operation: {}", std::mem::size_of::<Operation>());
        println!("Operand: {}", std::mem::size_of::<Operand>());
        println!("Instruction: {}", std::mem::size_of::<Instruction>());
    }
}
