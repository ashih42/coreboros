use anyhow::Result;

use crate::{
    instruction::{
        Instruction, addressing_mode::AddressingMode, modifier::Modifier, opcode::Opcode,
        operand::Operand, operation::Operation,
    },
    parser::{label_dictionary::LabelDictionary, operand_buffer::OperandBuffer},
};

#[derive(Debug)]
pub struct InstructionBuilder {
    pub opcode: Opcode,
    pub modifier: Option<Modifier>,
    pub operand_1: OperandBuffer,
    pub operand_2: Option<OperandBuffer>,
}

impl InstructionBuilder {
    /// Consume itself and build an `Instruction` if possible.
    ///
    /// # Errors
    /// Will return `Err` if:
    /// - `operand_1` fails to build.
    /// - `operand_2` fails to build.
    pub fn build(
        self,
        label_dictionary: &LabelDictionary,
        current_line: usize,
    ) -> Result<Instruction> {
        let operand_1 = self.operand_1.build(label_dictionary, current_line)?;

        let operand_2 = self
            .operand_2
            .map(|operand| operand.build(label_dictionary, current_line))
            .transpose()?;

        let (a, b) = Self::resolve_operands_a_b(self.opcode, operand_1, operand_2);

        let modifier = self
            .modifier
            .unwrap_or_else(|| Self::resolve_default_modifier(self.opcode, a.mode, b.mode));

        let operation = Operation::new(self.opcode, modifier);
        let instruction = Instruction::new(operation, a, b);

        Ok(instruction)
    }

    /// Given 1 or 2 operands, determine what values go into operands `a` and `b`, depending on `opcode`.
    const fn resolve_operands_a_b(
        opcode: Opcode,
        operand_1: Operand,
        operand_2: Option<Operand>,
    ) -> (Operand, Operand) {
        match operand_2 {
            Some(operand_2) => (operand_1, operand_2),
            None => match opcode {
                Opcode::DAT => (Operand::direct(0), operand_1),
                _ => (operand_1, Operand::direct(0)),
            },
        }
    }

    /// Determine the default `modifier` based on the given `opcode`, `a_mode`, and `b_mode`.
    /// See ICWS'88 to ICWS'94 Conversion: <https://corewar.co.uk/standards/icws94.htm#A2.1.2>
    /// Reference: <https://corewars.org/docs/guide.html#start_modif>
    fn resolve_default_modifier(
        opcode: Opcode,
        a_mode: AddressingMode,
        b_mode: AddressingMode,
    ) -> Modifier {
        use AddressingMode::Immediate;
        use Modifier::{AB, B, F, I};
        use Opcode::{
            ADD, CMP, DAT, DIV, DJN, JMN, JMP, JMZ, LDP, MOD, MOV, MUL, NOP, SEQ, SLT, SNE, SPL,
            STP, SUB,
        };

        match opcode {
            DAT | NOP => F,
            MOV | SEQ | SNE | CMP => {
                if a_mode == Immediate {
                    AB
                } else if b_mode == Immediate {
                    B
                } else {
                    I
                }
            }
            ADD | SUB | MUL | DIV | MOD => {
                if a_mode == Immediate {
                    AB
                } else if b_mode == Immediate {
                    B
                } else {
                    F
                }
            }
            SLT | LDP | STP => {
                if a_mode == Immediate {
                    AB
                } else {
                    B
                }
            }
            JMP | JMZ | JMN | DJN | SPL => B,
        }
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::expr::Expr;

    #[test]
    fn test_build_dat_5() {
        let instruction_builder = InstructionBuilder {
            opcode: Opcode::DAT,
            modifier: None,
            operand_1: OperandBuffer::new(None, Expr::Integer("5".to_owned())),
            operand_2: None,
        };
        let label_dict = LabelDictionary::default();

        let instruction = instruction_builder.build(&label_dict, 0).unwrap();

        assert!(matches!(
            instruction,
            Instruction {
                operation: Operation {
                    opcode: Opcode::DAT,
                    modifier: Modifier::F,
                },
                a: Operand {
                    mode: AddressingMode::Direct,
                    number: 0
                },
                b: Operand {
                    mode: AddressingMode::Direct,
                    number: 5
                },
            }
        ));
    }
}
