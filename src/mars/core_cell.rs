use crate::{
    instruction::Instruction, mars::instruction_cache::InstructionCache,
    warrior::warrior_id::WarriorId,
};

#[derive(Clone)]
pub struct CoreCell {
    pub instruction: Instruction,
    pub instruction_cache: InstructionCache,

    pub operation_author: Option<WarriorId>,
    pub a_author: Option<WarriorId>,
    pub b_author: Option<WarriorId>,
}

impl Default for CoreCell {
    fn default() -> Self {
        let instruction = Instruction::dat(0);

        Self {
            instruction,
            instruction_cache: (&instruction).into(),
            operation_author: None,
            a_author: None,
            b_author: None,
        }
    }
}

impl CoreCell {
    pub fn new(instruction: Instruction, author: Option<WarriorId>) -> Self {
        Self {
            instruction_cache: (&instruction).into(),
            instruction,
            operation_author: author,
            a_author: author,
            b_author: author,
        }
    }

    pub fn set_instruction(&mut self, instruction: Instruction, warrior_id: WarriorId) {
        self.instruction_cache = (&instruction).into();
        self.instruction = instruction;
        self.operation_author = Some(warrior_id);
        self.a_author = Some(warrior_id);
        self.b_author = Some(warrior_id);
    }

    pub fn set_a_number(&mut self, a_number: i32, warrior_id: WarriorId) {
        self.instruction.a.number = a_number;
        self.instruction_cache.a = a_number.to_string();
        self.a_author = Some(warrior_id);
    }

    pub fn set_b_number(&mut self, b_number: i32, warrior_id: WarriorId) {
        self.instruction.b.number = b_number;
        self.instruction_cache.b = b_number.to_string();
        self.b_author = Some(warrior_id);
    }

    pub fn clear_author(&mut self) {
        self.operation_author = None;
        self.a_author = None;
        self.b_author = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inspect_sizes() {
        println!("CoreCell: {}", std::mem::size_of::<CoreCell>()); // 144
        println!("TempCell: {}", std::mem::size_of::<TempCell>()); // 104

        println!("u8: {}", std::mem::size_of::<u8>());
        println!("Option<u8>: {}", std::mem::size_of::<Option<u8>>());
    }
}

pub struct TempCell {
    pub instruction: Instruction,
    pub instruction_cache: InstructionCache,

    pub operation_author: Option<u8>,
    pub a_author: Option<u8>,
    pub b_author: Option<u8>,
}
