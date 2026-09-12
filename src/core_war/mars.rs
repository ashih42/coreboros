use crate::{
    core_war::{
        config::Config,
        core_number::CoreNumber,
        mars::{
            core::Core, core_cell::CoreCell, core_instruction::CoreInstruction,
            task_outcome::TaskOutcome, task_queue::TaskQueue,
            warrior_placement_planner::WarriorPlacementPlanner,
        },
    },
    instruction::{addressing_mode::AddressingMode, opcode::Opcode},
    warrior::{Warrior, warrior_id::WarriorId},
};

pub mod core_instruction;

mod cell_slot_author;
mod core;
mod core_cell;
mod math_executor;
mod opcode_executor;
mod task_outcome;
mod task_queue;
mod warrior_placement_planner;

/// `Mars` ("Memory Array Redcode Simulator") is the virtual machine that executes Redcode instructions.
pub struct Mars {
    pub core: Core,
    task_queues: Box<[TaskQueue]>,
    warrior_placement_planner: WarriorPlacementPlanner,
}

impl Mars {
    /// Precondition: `ConfigManager` has already validated these `warriors` can fit on the core with the given `config`.
    pub fn new(warriors: &[Warrior], config: &Config) -> Self {
        let core = Core::new(config);

        let task_queues =
            std::iter::repeat_with(|| TaskQueue::with_capacity(config.task_queue_capacity))
                .take(warriors.len())
                .collect();

        let warrior_placement_planner = WarriorPlacementPlanner::new(config);

        let mut mars = Self {
            core,
            task_queues,
            warrior_placement_planner,
        };

        mars.load_warriors(warriors);
        mars
    }

    #[inline]
    pub const fn get_num_warriors(&self) -> usize {
        self.task_queues.len()
    }

    /// Initialize `Mars` with data from these `warriors`:
    /// - Load warriors' instructions to core.
    /// - Populate task queues with initial tasks.
    fn load_warriors(&mut self, warriors: &[Warrior]) {
        let starting_addresses = self
            .warrior_placement_planner
            .determine_placements(&self.core, warriors);

        for ((warrior_id, warrior), starting_address) in
            WarriorId::list_all_warrior_ids(warriors.len())
                .zip(warriors)
                .zip(starting_addresses)
        {
            self.load_instructions_to_core(warrior_id, warrior, starting_address);
            self.spawn_initial_task(warrior_id, warrior, starting_address);
        }
    }

    /// Load a specific warrior's instructions to the core.
    fn load_instructions_to_core(
        &mut self,
        warrior_id: WarriorId,
        warrior: &Warrior,
        starting_address: CoreNumber,
    ) {
        for (i, instruction) in warrior.instructions.iter().enumerate() {
            #[allow(
                clippy::arithmetic_side_effects,
                reason = "This expression cannot cause overflow."
            )]
            let address =
                CoreNumber::from_usize(starting_address.as_index() + i, self.core.get_size());

            *self.core.get_cell_mut(address) = CoreCell::new(
                CoreInstruction::from_instruction(instruction, self.core.get_size()),
                Some(warrior_id),
            );
        }
    }

    /// Get the `TaskQueue` corresponding to `warrior_id`.
    pub fn get_task_queue(&self, warrior_id: WarriorId) -> &TaskQueue {
        let index = warrior_id.as_index();

        #[allow(clippy::indexing_slicing, reason = "The index is valid 👌")]
        &self.task_queues[index]
    }

    /// Get the mutable `TaskQueue` corresponding to `warrior_id`.
    pub fn get_task_queue_mut(&mut self, warrior_id: WarriorId) -> &mut TaskQueue {
        let index = warrior_id.as_index();

        #[allow(clippy::indexing_slicing, reason = "The index is valid 👌")]
        &mut self.task_queues[index]
    }

    /// Spawn the initial task for a specific warrior.
    fn spawn_initial_task(
        &mut self,
        warrior_id: WarriorId,
        warrior: &Warrior,
        starting_position: CoreNumber,
    ) {
        #[allow(
            clippy::arithmetic_side_effects,
            reason = "This expression cannot cause overflow."
        )]
        let task = CoreNumber::from_usize(
            starting_position.as_index() + warrior.origin,
            self.core.get_size(),
        );

        self.get_task_queue_mut(warrior_id).push_if_not_full(task);
    }

    /// Reset the core and task queues, and load warriors for a new game.
    pub fn reset(&mut self, warriors: &[Warrior]) {
        self.core.reset();

        for task_queue in &mut self.task_queues {
            task_queue.clear();
        }

        self.load_warriors(warriors);
    }

    /// Execute one instruction for the given `warrior_id`.
    /// Note: The current instruction to execute is cached. This is an important, as the values in the address
    /// containing the current instruction could be mutated in the middle of executing this instruction.
    #[allow(clippy::indexing_slicing, reason = "The index is valid.")]
    #[allow(clippy::arithmetic_side_effects, reason = "`cycle_counter` is small.")]
    pub fn step(&mut self, warrior_id: WarriorId) {
        if let Some(address) = self.get_task_queue_mut(warrior_id).pop() {
            let instruction = self.core.get_cell(address).instruction;
            let outcome = self.execute_instruction(&instruction, address, warrior_id);
            self.process_outcome(outcome, warrior_id);
        }
    }

    /// Push the new task(s) in `outcome` into the given warrior's task queue.
    #[allow(
        clippy::needless_pass_by_value,
        reason = "This is the correct place to consume `outcome`."
    )]
    pub fn process_outcome(&mut self, outcome: TaskOutcome, warrior_id: WarriorId) {
        let task_queue = self.get_task_queue_mut(warrior_id);

        match outcome {
            TaskOutcome::Spawned {
                current_task,
                new_task,
            } => {
                task_queue.push_if_not_full(current_task);
                task_queue.push_if_not_full(new_task);
            }
            TaskOutcome::Lived { current_task } => {
                task_queue.push_if_not_full(current_task);
            }
            TaskOutcome::Died => (),
        }
    }

    /// Handle pre-decrement, then execute the instruction, then handle post-increment.
    fn execute_instruction(
        &mut self,
        instruction: &CoreInstruction,
        address: CoreNumber,
        warrior_id: WarriorId,
    ) -> TaskOutcome {
        self.write_pre_decrement(instruction, address, warrior_id);

        let outcome = self.execute_by_opcode(instruction, address, warrior_id);

        self.write_post_increment(instruction, address, warrior_id);

        outcome
    }

    /// Check if `instruction` does pre-decrement in its A mode or B mode, and update the target cell's A or B field if applicable.
    fn write_pre_decrement(
        &mut self,
        instruction: &CoreInstruction,
        address: CoreNumber,
        warrior_id: WarriorId,
    ) {
        // Update the indirect A cell's A or B field if applicable.
        let a_indirect_address = self.core.resolve_address(address, instruction.a.number);

        match instruction.a.mode {
            AddressingMode::AIndirectPreDecrement => {
                self.core.decrement_a_number(a_indirect_address, warrior_id);
            }
            AddressingMode::BIndirectPreDecrement => {
                self.core.decrement_b_number(a_indirect_address, warrior_id);
            }
            _ => (),
        }

        // Update the indirect B cell's A or B field if applicable.
        let b_indirect_address = self.core.resolve_address(address, instruction.b.number);

        match instruction.b.mode {
            AddressingMode::AIndirectPreDecrement => {
                self.core.decrement_a_number(b_indirect_address, warrior_id);
            }
            AddressingMode::BIndirectPreDecrement => {
                self.core.decrement_b_number(b_indirect_address, warrior_id);
            }
            _ => (),
        }
    }

    /// Check if `instruction` does post-decrement in its A mode or B mode, and update the target cell's A or B field if applicable.
    fn write_post_increment(
        &mut self,
        instruction: &CoreInstruction,
        address: CoreNumber,
        warrior_id: WarriorId,
    ) {
        let a_indirect_address = self.core.resolve_address(address, instruction.a.number);

        match instruction.a.mode {
            AddressingMode::AIndirectPostIncrement => {
                self.core.increment_a_number(a_indirect_address, warrior_id);
            }
            AddressingMode::BIndirectPostIncrement => {
                self.core.increment_b_number(a_indirect_address, warrior_id);
            }
            _ => (),
        }

        let b_indirect_address = self.core.resolve_address(address, instruction.b.number);

        match instruction.b.mode {
            AddressingMode::AIndirectPostIncrement => {
                self.core.increment_a_number(b_indirect_address, warrior_id);
            }
            AddressingMode::BIndirectPostIncrement => {
                self.core.increment_b_number(b_indirect_address, warrior_id);
            }
            _ => (),
        }
    }

    /// Execute the instruction by calling the function corresponding to its `opcode`.
    fn execute_by_opcode(
        &mut self,
        instruction: &CoreInstruction,
        address: CoreNumber,
        warrior_id: WarriorId,
    ) -> TaskOutcome {
        use opcode_executor as exec;

        let core = &mut self.core;

        match instruction.operation.opcode {
            Opcode::DAT => exec::exec_dat(instruction, address, core, warrior_id),

            Opcode::MOV => exec::exec_mov(instruction, address, core, warrior_id),

            Opcode::ADD => exec::exec_add(instruction, address, core, warrior_id),
            Opcode::SUB => exec::exec_sub(instruction, address, core, warrior_id),
            Opcode::MUL => exec::exec_mul(instruction, address, core, warrior_id),
            Opcode::DIV => exec::exec_div(instruction, address, core, warrior_id),
            Opcode::MOD => exec::exec_mod(instruction, address, core, warrior_id),

            Opcode::JMP => exec::exec_jmp(instruction, address, core, warrior_id),
            Opcode::JMZ => exec::exec_jmz(instruction, address, core, warrior_id),
            Opcode::JMN => exec::exec_jmn(instruction, address, core, warrior_id),
            Opcode::DJN => exec::exec_djn(instruction, address, core, warrior_id),

            Opcode::SPL => exec::exec_spl(instruction, address, core, warrior_id),

            // Note: `CMP` is just an alias for `SEQ`.
            Opcode::CMP | Opcode::SEQ => exec::exec_seq(instruction, address, core, warrior_id),
            Opcode::SNE => exec::exec_sne(instruction, address, core, warrior_id),
            Opcode::SLT => exec::exec_slt(instruction, address, core, warrior_id),

            // Note: `LDP` and `STP` are currently not implemented, so they are equivalent to `NOP`.
            Opcode::LDP | Opcode::STP | Opcode::NOP => {
                exec::exec_nop(instruction, address, core, warrior_id)
            }
        }
    }
}
