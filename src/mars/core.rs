use crate::{
    instruction::{Instruction, addressing_mode::AddressingMode, operand::Operand},
    mars::{
        address::Address, config::core_initialization_strategy::CoreInitializationStrategy,
        core_cell::CoreCell, math_executor::MathExecutor,
    },
    warrior::warrior_id::WarriorId,
};

pub struct Core {
    pub cells: Vec<CoreCell>,
    initialization_strategy: CoreInitializationStrategy,
    pub width: usize,
    pub height: usize,
    core_size: usize,
    pub math_executor: MathExecutor,
}

impl Core {
    pub fn new(
        width: usize,
        height: usize,
        initialization_strategy: CoreInitializationStrategy,
    ) -> Self {
        let core_size = width * height;

        let cells = match initialization_strategy {
            CoreInitializationStrategy::FillDat00 | CoreInitializationStrategy::Leftover => {
                vec![CoreCell::default(); core_size]
            }
            CoreInitializationStrategy::Random => std::iter::repeat_with(|| {
                CoreCell::new(Instruction::random_instruction_wrapped(core_size), None)
            })
            .take(core_size)
            .collect(),
        };

        Self {
            cells,
            initialization_strategy,
            width,
            height,
            core_size,
            math_executor: MathExecutor::new(core_size),
        }
    }

    pub fn reset(&mut self) {
        match self.initialization_strategy {
            CoreInitializationStrategy::FillDat00 => {
                self.cells.fill(CoreCell::default());
            }
            CoreInitializationStrategy::Leftover => {
                for cell in &mut self.cells {
                    cell.clear_author();
                }
            }
            CoreInitializationStrategy::Random => {
                let core_size = self.size();

                self.cells.fill_with(|| {
                    CoreCell::new(Instruction::random_instruction_wrapped(core_size), None)
                });
            }
        }
    }

    /// This function requires `address` to be valid.
    #[inline]
    pub fn get_cell(&self, address: Address) -> &CoreCell {
        &self.cells[address]
    }

    /// This function requires `address` to be valid.
    #[inline]
    pub fn get_cell_mut(&mut self, address: Address) -> &mut CoreCell {
        &mut self.cells[address]
    }

    pub fn get_cell_with_wraparound(&self, address: Address) -> &CoreCell {
        let position = address % self.size();
        &self.cells[position]
    }

    /// This function requires `address` to be valid.
    pub fn wrap_and_load_instruction(
        &mut self,
        address: Address,
        instruction: &Instruction,
        author: Option<WarriorId>,
    ) {
        let wrapped_instruction = self.math_executor.wrap_instruction(instruction);

        self.cells[address] = CoreCell::new(wrapped_instruction, author);
    }

    /// This function requires `address` to be valid.
    pub fn increment_a_number(&mut self, address: Address, author: WarriorId) {
        let a_number = self.get_cell(address).instruction.a.number;
        let a_number = self.math_executor.increment(a_number);

        let cell = self.get_cell_mut(address);
        cell.set_a_number(a_number, author);
    }

    /// This function requires `address` to be valid.
    pub fn increment_b_number(&mut self, address: Address, author: WarriorId) {
        let b_number = self.get_cell(address).instruction.b.number;
        let b_number = self.math_executor.increment(b_number);

        let cell = self.get_cell_mut(address);
        cell.set_b_number(b_number, author);
    }

    /// This function requires `address` to be valid.
    pub fn decrement_a_number(&mut self, address: Address, author: WarriorId) {
        let a_number = self.get_cell(address).instruction.a.number;
        let a_number = self.math_executor.decrement(a_number);

        let cell = self.get_cell_mut(address);
        cell.set_a_number(a_number, author);
    }

    /// This function requires `address` to be valid.
    pub fn decrement_b_number(&mut self, address: Address, author: WarriorId) {
        let b_number = self.get_cell(address).instruction.b.number;
        let b_number = self.math_executor.decrement(b_number);

        let cell = self.get_cell_mut(address);
        cell.set_b_number(b_number, author);
    }

    #[inline]
    pub const fn size(&self) -> usize {
        self.core_size
    }

    pub fn resolve_address(&self, address: Address, offset: i32) -> Address {
        let destination = (address as i32) + offset;
        destination.rem_euclid(self.size() as i32) as Address
    }

    /// Determine the address specified in the `operand`.
    /// Pre-decrement and Post-increment operations are handled before/after this function,
    /// so here it is okay to resolve these variations of indirect modes in the same way.
    pub fn resolve_operand_address(&self, operand: &Operand, current_address: Address) -> Address {
        use AddressingMode as AM;

        let indirect_address = self.resolve_address(current_address, operand.number);
        let indirect_cell = self.get_cell(indirect_address);

        match operand.mode {
            AM::Immediate => current_address,
            AM::Direct => self.resolve_address(current_address, operand.number),
            AM::AIndirect | AM::AIndirectPreDecrement | AM::AIndirectPostIncrement => {
                self.resolve_address(indirect_address, indirect_cell.instruction.a.number)
            }
            AM::BIndirect | AM::BIndirectPreDecrement | AM::BIndirectPostIncrement => {
                self.resolve_address(indirect_address, indirect_cell.instruction.b.number)
            }
        }
    }

    pub fn resolve_instruction_a_b(
        &self,
        current_address: Address,
        operand: &Operand,
    ) -> (Instruction, i32, i32) {
        let address = self.resolve_operand_address(operand, current_address);
        let instruction = self.get_cell(address).instruction;

        let a_number = match operand.mode {
            AddressingMode::Immediate => operand.number,
            _ => instruction.a.number,
        };

        let b_number = match operand.mode {
            AddressingMode::Immediate => 0,
            _ => instruction.b.number,
        };

        (instruction, a_number, b_number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_address() {
        let core = Core::new(10, 10, CoreInitializationStrategy::FillDat00);

        assert_eq!(core.resolve_address(0, 1), 1);
        assert_eq!(core.resolve_address(0, 1005), 5);
        assert_eq!(core.resolve_address(0, -1001), 99);
    }
}
