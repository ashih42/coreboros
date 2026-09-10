use crate::{
    instruction::{Instruction, addressing_mode::AddressingMode, opcode::Opcode},
    mars::{
        address::Address,
        config::{Config, warrior_separation_strategy::WarriorSeparationStrategy},
        core::Core,
        task_outcome::TaskOutcome,
        task_queue::TaskQueue,
    },
    rng,
    warrior::{Warrior, warrior_id::WarriorId},
};

pub mod address;
pub mod config;

mod cell_slot_author;
mod core;
mod core_cell;
mod math_executor;
mod opcode_executor;
mod task_outcome;
mod task_queue;

/// `Mars` ("Memory Array Redcode Simulator") is the virtual machine that executes Redcode instructions.
pub struct Mars {
    pub core: Core,
    pub task_queues: Box<[TaskQueue]>,
}

impl Mars {
    /// Precondition: `ConfigManager` has already validated these `warriors` can fit on the core with the given `config`.
    pub fn new(warriors: &[Warrior], config: &Config) -> Self {
        let core = Core::new(config);

        let task_queues =
            std::iter::repeat_with(|| TaskQueue::with_capacity(config.task_queue_capacity))
                .take(warriors.len())
                .collect::<Vec<_>>()
                .into_boxed_slice();

        let mut mars = Self { core, task_queues };

        mars.load_warriors_to_core_and_initialize_task_queues(warriors, config);
        mars
    }

    #[inline]
    pub const fn get_num_warriors(&self) -> usize {
        self.task_queues.len()
    }

    /// Load each warrior's instructions to core and initialize each warrior's task queue with the first task.
    #[allow(clippy::indexing_slicing, reason = "The index is valid.")]
    #[allow(clippy::arithmetic_side_effects, reason = "The numbers are small.")]
    fn load_warriors_to_core_and_initialize_task_queues(
        &mut self,
        warriors: &[Warrior],
        config: &Config,
    ) {
        let core_size = self.core.get_size();
        let starting_positions = self.determine_starting_positions(warriors, config);

        for (warrior_id, warrior) in warriors.iter().enumerate() {
            let starting_position = starting_positions[warrior_id];

            // Copy instructions to core.
            for (i, instruction) in warrior.instructions.iter().enumerate() {
                let position = (starting_position + i) % core_size;

                self.core
                    .wrap_and_load_instruction(position, instruction, Some(warrior_id));
            }

            // Push initial task.
            let task = (starting_position + warrior.origin) % core_size;
            self.task_queues[warrior_id].push_if_not_full(task);
        }
    }

    fn determine_starting_positions(&self, warriors: &[Warrior], config: &Config) -> Vec<usize> {
        match config.warrior_separation_strategy {
            WarriorSeparationStrategy::Equal => self.determine_starting_positions_equal(),
            WarriorSeparationStrategy::Random => {
                self.determine_starting_positions_random(warriors, config)
            }
        }
    }

    /// Determine the starting positions under the `Equal` warrior separation strategy.
    fn determine_starting_positions_equal(&self) -> Vec<usize> {
        let core_size = self.core.get_size();

        #[allow(clippy::arithmetic_side_effects, reason = "These numbers are small.")]
        (0..self.get_num_warriors())
            .map(|warrior_id| core_size / self.get_num_warriors() * warrior_id)
            .collect()
    }

    /// Determine the starting positions under the `Random` warrior separation strategy.
    /// Note: This additionally shuffles the position order at the end, for even more randomization.
    #[allow(clippy::indexing_slicing, reason = "The index is valid.")]
    #[allow(clippy::arithmetic_side_effects, reason = "The numbers are small.")]
    fn determine_starting_positions_random(
        &self,
        warriors: &[Warrior],
        config: &Config,
    ) -> Vec<usize> {
        let core_size = self.core.get_size();

        let instruction_lengths = warriors
            .iter()
            .map(|warrior| warrior.instructions.len())
            .collect::<Vec<_>>();

        let separation_buckets = {
            let total_instructions = warriors
                .iter()
                .map(|warrior| warrior.instructions.len())
                .sum::<usize>();

            let mut buckets = vec![config.min_distance_between_warriors; self.get_num_warriors()];

            #[allow(
                clippy::suspicious_operation_groupings,
                reason = "This is correct and intended."
            )]
            let mut remaining_cells = core_size
                - total_instructions
                - (config.min_distance_between_warriors * self.get_num_warriors());

            while remaining_cells != 0 {
                let bucket_id = rng::rand_range(0, self.get_num_warriors());
                buckets[bucket_id] += 1;
                remaining_cells -= 1;
            }

            buckets
        };

        let mut positions = Vec::with_capacity(self.get_num_warriors());
        let mut position = 0;

        for (instructions, separation) in instruction_lengths.iter().zip(separation_buckets.iter())
        {
            positions.push(position);
            position += instructions + separation;
        }

        rng::shuffle(&mut positions);
        positions
    }

    /// Reset the core and task queues, and load warriors' instructions to core for a new game.
    pub fn reset(&mut self, warriors: &[Warrior], config: &Config) {
        self.core.reset();

        for task_queue in &mut self.task_queues {
            task_queue.clear();
        }

        self.load_warriors_to_core_and_initialize_task_queues(warriors, config);
    }

    /// Execute one instruction.
    #[allow(clippy::indexing_slicing, reason = "The index is valid.")]
    #[allow(clippy::arithmetic_side_effects, reason = "`cycle_counter` is small.")]
    pub fn step(&mut self, current_warrior_id: WarriorId) {
        Self::execute_task(
            current_warrior_id,
            &mut self.task_queues[current_warrior_id],
            &mut self.core,
        );
    }

    /// Pop off one task, execute it, and push resulting new task(s) back to the queue.
    fn execute_task(warrior_id: WarriorId, task_queue: &mut TaskQueue, core: &mut Core) {
        if let Some(address) = task_queue.pop() {
            let instruction = core.get_cell(address).instruction; // Cache the current instruction to be executed.

            // macroquad::prelude::info!(
            //     "Warrior {} executes at address {:>4}:\t{}\t{}\t{}",
            //     warrior_id, address, &instruction.operation, &instruction.a, &instruction.b
            // );

            match Self::execute_instruction(&instruction, address, core, warrior_id) {
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
    }

    /// Check if `instruction` does pre-decrement in its A mode or B mode, and update the target cell's A or B field if applicable.
    fn write_pre_decrement(
        instruction: &Instruction,
        address: Address,
        core: &mut Core,
        warrior_id: WarriorId,
    ) {
        // Update the indirect A cell's A or B field if applicable.
        let a_indirect_address = core.resolve_address(address, instruction.a.number);

        match instruction.a.mode {
            AddressingMode::AIndirectPreDecrement => {
                core.decrement_a_number(a_indirect_address, warrior_id);
            }
            AddressingMode::BIndirectPreDecrement => {
                core.decrement_b_number(a_indirect_address, warrior_id);
            }
            _ => (),
        }

        // Update the indirect B cell's A or B field if applicable.
        let b_indirect_address = core.resolve_address(address, instruction.b.number);

        match instruction.b.mode {
            AddressingMode::AIndirectPreDecrement => {
                core.decrement_a_number(b_indirect_address, warrior_id);
            }
            AddressingMode::BIndirectPreDecrement => {
                core.decrement_b_number(b_indirect_address, warrior_id);
            }
            _ => (),
        }
    }

    /// Check if `instruction` does post-decrement in its A mode or B mode, and update the target cell's A or B field if applicable.
    fn write_post_increment(
        instruction: &Instruction,
        address: Address,
        core: &mut Core,
        warrior_id: WarriorId,
    ) {
        let a_indirect_address = core.resolve_address(address, instruction.a.number);

        match instruction.a.mode {
            AddressingMode::AIndirectPostIncrement => {
                core.increment_a_number(a_indirect_address, warrior_id);
            }
            AddressingMode::BIndirectPostIncrement => {
                core.increment_b_number(a_indirect_address, warrior_id);
            }
            _ => (),
        }

        let b_indirect_address = core.resolve_address(address, instruction.b.number);

        match instruction.b.mode {
            AddressingMode::AIndirectPostIncrement => {
                core.increment_a_number(b_indirect_address, warrior_id);
            }
            AddressingMode::BIndirectPostIncrement => {
                core.increment_b_number(b_indirect_address, warrior_id);
            }
            _ => (),
        }
    }

    /// Handle pre-decrement, then execute the instruction, then handle post-increment.
    fn execute_instruction(
        instruction: &Instruction,
        address: Address,
        core: &mut Core,
        warrior_id: WarriorId,
    ) -> TaskOutcome {
        Self::write_pre_decrement(instruction, address, core, warrior_id);

        let outcome = Self::execute_by_opcode(instruction, address, core, warrior_id);

        Self::write_post_increment(instruction, address, core, warrior_id);

        outcome
    }

    /// Execute the instruction by calling the function corresponding to its `opcode`.
    fn execute_by_opcode(
        instruction: &Instruction,
        address: Address,
        core: &mut Core,
        warrior_id: WarriorId,
    ) -> TaskOutcome {
        use opcode_executor as exec;

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
