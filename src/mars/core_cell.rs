use crate::{
    instruction::Instruction, mars::cell_slot_author::CellSlotAuthor,
    warrior::warrior_id::WarriorId,
};

/// `CoreCell` represents a concrete value written in the core at a specific address.
#[derive(Clone)]
pub struct CoreCell {
    pub instruction: Instruction,
    pub operation_author: CellSlotAuthor,
    pub a_author: CellSlotAuthor,
    pub b_author: CellSlotAuthor,
}

impl Default for CoreCell {
    fn default() -> Self {
        let instruction = Instruction::dat(0);

        Self {
            instruction,
            operation_author: CellSlotAuthor::None,
            a_author: CellSlotAuthor::None,
            b_author: CellSlotAuthor::None,
        }
    }
}

impl CoreCell {
    pub fn new(instruction: Instruction, author: Option<WarriorId>) -> Self {
        Self {
            instruction,
            operation_author: author.into(),
            a_author: author.into(),
            b_author: author.into(),
        }
    }

    pub fn set_instruction(&mut self, instruction: Instruction, warrior_id: WarriorId) {
        let author = Some(warrior_id).into();

        self.instruction = instruction;
        self.operation_author = author;
        self.a_author = author;
        self.b_author = author;
    }

    pub fn set_a_number(&mut self, a_number: i32, warrior_id: WarriorId) {
        self.instruction.a.number = a_number;
        self.a_author = Some(warrior_id).into();
    }

    pub fn set_b_number(&mut self, b_number: i32, warrior_id: WarriorId) {
        self.instruction.b.number = b_number;
        self.b_author = Some(warrior_id).into();
    }

    pub const fn clear_author(&mut self) {
        self.operation_author = CellSlotAuthor::None;
        self.a_author = CellSlotAuthor::None;
        self.b_author = CellSlotAuthor::None;
    }
}
