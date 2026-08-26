use std::fmt;

use crate::{
    instruction::{
        addressing_mode::AddressingMode, modifier::Modifier, opcode::Opcode, operand::Operand,
        operation::Operation,
    },
    rng,
};

pub mod addressing_mode;
pub mod modifier;
pub mod opcode;
pub mod operand;
pub mod operation;

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

    pub const fn dat(number: i32) -> Self {
        Self {
            operation: Operation {
                opcode: Opcode::DAT,
                modifier: Modifier::F,
            },
            a: Operand::direct(0),
            b: Operand::direct(number),
        }
    }

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

    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        clippy::as_conversions,
        reason = "These conversions are safe because the numbers are small."
    )]
    pub fn random_instruction_wrapped(core_size: usize) -> Self {
        let opcode = Opcode::random_opcode();
        let modifier = Modifier::random_modifier();

        let a_mode = AddressingMode::random_addressing_mode();
        let a_number = rng::rand_range(0, core_size) as i32;

        let b_mode = AddressingMode::random_addressing_mode();
        let b_number = rng::rand_range(0, core_size) as i32;

        Self {
            operation: Operation::new(opcode, modifier),
            a: Operand::new(a_mode, a_number),
            b: Operand::new(b_mode, b_number),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::addressing_mode::AddressingMode;

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
