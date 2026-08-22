use crate::{
    instruction::Instruction,
    mars::{cell_slot_author::CellSlotAuthor, instruction_cache::InstructionCache},
    warrior::warrior_id::WarriorId,
};

#[derive(Clone)]
pub struct CoreCell {
    pub instruction: Instruction,
    pub instruction_cache: InstructionCache,

    pub operation_author: CellSlotAuthor,
    pub a_author: CellSlotAuthor,
    pub b_author: CellSlotAuthor,
}

impl Default for CoreCell {
    fn default() -> Self {
        let instruction = Instruction::dat(0);

        Self {
            instruction,
            instruction_cache: (&instruction).into(),
            operation_author: CellSlotAuthor::None,
            a_author: CellSlotAuthor::None,
            b_author: CellSlotAuthor::None,
        }
    }
}

impl CoreCell {
    pub fn new(instruction: Instruction, author: Option<WarriorId>) -> Self {
        Self {
            instruction_cache: (&instruction).into(),
            instruction,
            operation_author: author.into(),
            a_author: author.into(),
            b_author: author.into(),
        }
    }

    pub fn set_instruction(&mut self, instruction: Instruction, warrior_id: WarriorId) {
        let author = Some(warrior_id).into();

        self.instruction_cache = (&instruction).into();
        self.instruction = instruction;
        self.operation_author = author;
        self.a_author = author;
        self.b_author = author;
    }

    pub fn set_a_number(&mut self, a_number: i32, warrior_id: WarriorId) {
        self.instruction.a.number = a_number;
        self.instruction_cache.a = a_number.to_string();
        self.a_author = Some(warrior_id).into();
    }

    pub fn set_b_number(&mut self, b_number: i32, warrior_id: WarriorId) {
        self.instruction.b.number = b_number;
        self.instruction_cache.b = b_number.to_string();
        self.b_author = Some(warrior_id).into();
    }

    pub fn clear_author(&mut self) {
        self.operation_author = CellSlotAuthor::None;
        self.a_author = CellSlotAuthor::None;
        self.b_author = CellSlotAuthor::None;
    }
}
