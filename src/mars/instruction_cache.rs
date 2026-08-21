use crate::instruction::Instruction;

#[derive(Debug, Clone)]
pub struct InstructionCache {
    pub operation: String,
    pub a: String,
    pub b: String,
}

impl From<&Instruction> for InstructionCache {
    fn from(instruction: &Instruction) -> Self {
        Self {
            operation: instruction.operation.to_string(),
            a: instruction.a.number.to_string(),
            b: instruction.b.number.to_string(),
        }
    }
}
