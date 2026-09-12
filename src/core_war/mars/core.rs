use crate::{
    core_war::{
        config::{Config, core_initialization_strategy::CoreInitializationStrategy},
        core_number::CoreNumber,
        mars::{
            core_cell::CoreCell,
            core_instruction::{CoreInstruction, core_operand::CoreOperand},
            math_executor::MathExecutor,
        },
    },
    instruction::addressing_mode::AddressingMode,
    warrior::warrior_id::WarriorId,
};

/// `Core` is the circular shared memory space occupied by all warriors' instructions.
pub struct Core {
    cells: Box<[CoreCell]>,
    initialization_strategy: CoreInitializationStrategy,
    pub math_executor: MathExecutor,
}

impl Core {
    pub fn new(config: &Config) -> Self {
        let core_size = config.core_dimension.as_size();
        let initialization_strategy = config.core_initialization_strategy;

        let cells = match initialization_strategy {
            CoreInitializationStrategy::FillDat00 | CoreInitializationStrategy::Leftover => {
                vec![CoreCell::default(); core_size].into_boxed_slice()
            }
            CoreInitializationStrategy::Random => std::iter::repeat_with(|| {
                CoreCell::new(CoreInstruction::random_instruction(core_size), None)
            })
            .take(core_size)
            .collect(),
        };

        Self {
            cells,
            initialization_strategy,
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
                let core_size = self.get_size();
                self.cells.fill_with(|| {
                    CoreCell::new(CoreInstruction::random_instruction(core_size), None)
                });
            }
        }
    }

    #[inline]
    pub const fn get_size(&self) -> usize {
        self.cells.len()
    }

    /// Note: `address` can be safely used as an index to access `cells`.
    #[inline]
    pub fn get_cell(&self, address: CoreNumber) -> &CoreCell {
        let index = address.as_index();

        #[allow(clippy::indexing_slicing, reason = "The index is valid 👌")]
        &self.cells[index]
    }

    /// Note: `address` can be safely used as an index to access `cells`.
    #[inline]
    pub fn get_cell_mut(&mut self, address: CoreNumber) -> &mut CoreCell {
        let index = address.as_index();

        #[allow(clippy::indexing_slicing, reason = "The index is valid 👌")]
        &mut self.cells[index]
    }

    /// Note: This function requires `address` to be valid.
    pub fn increment_a_number(&mut self, address: CoreNumber, author: WarriorId) {
        let a_number = self.get_cell(address).instruction.a.number;
        let a_number = self.math_executor.increment(a_number);

        let cell = self.get_cell_mut(address);
        cell.set_a_number(a_number, author);
    }

    /// Note: This function requires `address` to be valid.
    pub fn increment_b_number(&mut self, address: CoreNumber, author: WarriorId) {
        let b_number = self.get_cell(address).instruction.b.number;
        let b_number = self.math_executor.increment(b_number);

        let cell = self.get_cell_mut(address);
        cell.set_b_number(b_number, author);
    }

    /// Note: This function requires `address` to be valid.
    pub fn decrement_a_number(&mut self, address: CoreNumber, author: WarriorId) {
        let a_number = self.get_cell(address).instruction.a.number;
        let a_number = self.math_executor.decrement(a_number);

        let cell = self.get_cell_mut(address);
        cell.set_a_number(a_number, author);
    }

    /// Note: This function requires `address` to be valid.
    pub fn decrement_b_number(&mut self, address: CoreNumber, author: WarriorId) {
        let b_number = self.get_cell(address).instruction.b.number;
        let b_number = self.math_executor.decrement(b_number);

        let cell = self.get_cell_mut(address);
        cell.set_b_number(b_number, author);
    }

    pub const fn resolve_address(&self, address: CoreNumber, offset: CoreNumber) -> CoreNumber {
        #[allow(
            clippy::arithmetic_side_effects,
            reason = "This expression cannot cause overflow/underflow."
        )]
        let destination = address.as_i32() + offset.as_i32();
        let core_size = self.get_size();

        CoreNumber::from_i32(destination, core_size)
    }

    /// Determine the address specified in the `operand`.
    /// Pre-decrement and Post-increment operations are handled before/after this function,
    /// so here it is okay to resolve these variations of indirect modes in the same way.
    pub fn resolve_operand_address(
        &self,
        operand: CoreOperand,
        current_address: CoreNumber,
    ) -> CoreNumber {
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
        current_address: CoreNumber,
        operand: CoreOperand,
    ) -> (CoreInstruction, CoreNumber, CoreNumber) {
        let address = self.resolve_operand_address(operand, current_address);
        let instruction = self.get_cell(address).instruction;

        let a_number = match operand.mode {
            AddressingMode::Immediate => operand.number,
            _ => instruction.a.number,
        };

        let b_number = match operand.mode {
            AddressingMode::Immediate => CoreNumber::zero(),
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
        let config = Config::default();
        let core = Core::new(&config);
        let core_size = core.get_size();

        // Note: The correct values for this test depend on the premise that `core_size` is 80.
        assert_eq!(core_size, 80);

        assert_eq!(
            core.resolve_address(
                CoreNumber::from_i32(0, core_size),
                CoreNumber::from_i32(0, core_size)
            ),
            CoreNumber::from_i32(0, core_size)
        );

        assert_eq!(
            core.resolve_address(
                CoreNumber::from_i32(0, core_size),
                CoreNumber::from_i32(10, core_size)
            ),
            CoreNumber::from_i32(10, core_size)
        );

        assert_eq!(
            core.resolve_address(
                CoreNumber::from_i32(0, core_size),
                CoreNumber::from_i32(100, core_size)
            ),
            CoreNumber::from_i32(20, core_size)
        );

        assert_eq!(
            core.resolve_address(
                CoreNumber::from_i32(0, core_size),
                CoreNumber::from_i32(-1, core_size)
            ),
            CoreNumber::from_i32(79, core_size)
        );
    }
}
