use crate::{
    core_war::{core_number::CoreNumber, mars::core_instruction::core_operand::CoreOperand},
    instruction::{
        Instruction, addressing_mode::AddressingMode, modifier::Modifier, opcode::Opcode,
        operation::Operation,
    },
    rng,
};

pub mod core_operand;

/// `CoreInstruction` is a single assembly instruction with operand numbers as `CoreNumber`,
/// under the constraints of a specific `Core`.
///
/// Thus, this is a concrete instruction that can be stored in the `Core`, and it can be executed.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CoreInstruction {
    pub operation: Operation,
    pub a: CoreOperand,
    pub b: CoreOperand,
}

impl CoreInstruction {
    pub const fn dat_zero() -> Self {
        Self {
            operation: Operation {
                opcode: Opcode::DAT,
                modifier: Modifier::F,
            },
            a: CoreOperand::direct(CoreNumber::zero()),
            b: CoreOperand::direct(CoreNumber::zero()),
        }
    }

    /// Return a `CoreInstruction` with completely randomized opcode, modifier, A and B addressing modes and numbers.
    pub fn random_instruction(core_size: usize) -> Self {
        let opcode = Opcode::random_opcode();
        let modifier = Modifier::random_modifier();

        let a_mode = AddressingMode::random_addressing_mode();
        let a_number = CoreNumber::from_usize_unchecked(rng::rand_range(0, core_size));

        let b_mode = AddressingMode::random_addressing_mode();
        let b_number = CoreNumber::from_usize_unchecked(rng::rand_range(0, core_size));

        Self {
            operation: Operation::new(opcode, modifier),
            a: CoreOperand::new(a_mode, a_number),
            b: CoreOperand::new(b_mode, b_number),
        }
    }

    /// Create a concrete `CoreInstruction` from the given generalized `Instruction`,
    /// converting its operand `i32` numbers to `CoreNumber` numbers.
    pub const fn from_instruction(instruction: &Instruction, core_size: usize) -> Self {
        let a_number = CoreNumber::from_i32(instruction.a.number, core_size);
        let b_number = CoreNumber::from_i32(instruction.b.number, core_size);

        Self {
            operation: instruction.operation,
            a: CoreOperand::new(instruction.a.mode, a_number),
            b: CoreOperand::new(instruction.b.mode, b_number),
        }
    }
}
