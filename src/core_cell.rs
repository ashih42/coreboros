use crate::{instruction::Instruction, warrior::WarriorId};

#[derive(Clone)]
pub struct CoreCell {
    pub instruction: Instruction,
    pub instruction_author: Option<WarriorId>,
    pub a_author: Option<WarriorId>,
    pub b_author: Option<WarriorId>,
}

impl Default for CoreCell {
    fn default() -> Self {
        Self {
            instruction: Instruction::new_dat_f_0_0(),
            instruction_author: None,
            a_author: None,
            b_author: None,
        }
    }
}
