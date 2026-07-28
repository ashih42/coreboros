use std::fmt;

use crate::instruction::{
    modifier::Modifier, opcode::Opcode, operand::Operand, operation::Operation,
};

pub mod addressing_mode;
pub mod modifier;
pub mod opcode;
pub mod operand;
pub mod operation;

/*
DAT uses operand B (operand A is set to #0).

Other one-operand operators use operand A (operand B is set to #0).

I think these unused operands may be used by other instructions, so it is NOT appropriate to
record these optional operands as Option<Operand>

*/

#[derive(Clone, Copy)]
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

    #[must_use]
    pub fn to_load_file(&self) -> String {
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

    /// `DAT.F $0, $0` is used as a strategy to initialize the core.
    #[must_use]
    pub const fn new_dat_f_0_0() -> Self {
        Self {
            operation: Operation {
                opcode: Opcode::DAT,
                modifier: Modifier::F,
            },
            a: Operand::direct_zero(),
            b: Operand::direct_zero(),
        }
    }
}
