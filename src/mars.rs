use crate::{
    instruction::{Instruction, addressing_mode::AddressingMode, opcode::Opcode},
    mars::{
        address::Address,
        config::{Config, warrior_separation_strategy::WarriorSeparationStrategy},
        core::Core,
        opcode_executor::*,
        task_outcome::TaskOutcome,
        task_queue::TaskQueue,
        warrior_context::WarriorContext,
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
mod warrior_context;

pub struct Mars {
    pub config: Config,
    pub core: Core,
    pub warrior_contexts: Vec<WarriorContext>,
    pub game_counter: usize,
    pub turn_counter: usize,
    pub cycle_counter: usize,
    pub current_warrior_id: WarriorId,
    pub game_over: bool,
    pub winner: Option<WarriorId>,
}

impl Mars {
    /// Note: `ConfigManager` has validated these `warriors` can fit on the core with the given `config`.
    pub fn new(warriors: Vec<Warrior>, config: Config) -> Self {
        let (core_width, core_height) = config.core_dimension.as_grid_dimensions();

        let core = Core::new(core_width, core_height, config.core_initialization_strategy);

        let contexts = warriors
            .into_iter()
            .map(|warrior| {
                WarriorContext::new(
                    warrior,
                    TaskQueue::with_capacity(config.task_queue_capacity),
                )
            })
            .collect();

        let mut mars = Self {
            config,
            warrior_contexts: contexts,
            core,
            game_counter: 0,
            turn_counter: 0,
            cycle_counter: 0,
            current_warrior_id: 0,
            game_over: false,
            winner: None,
        };

        mars.load_warriors_to_core_and_initialize_task_queues();
        mars
    }

    fn load_warriors_to_core_and_initialize_task_queues(&mut self) {
        let starting_positions = self.determine_starting_positions();

        for (warrior_id, context) in self.warrior_contexts.iter_mut().enumerate() {
            let starting_position = starting_positions[warrior_id];

            // Copy instructions to core.
            for (i, instruction) in context.warrior.instructions.iter().enumerate() {
                let position = (starting_position + i) % self.core.size();

                self.core
                    .wrap_and_load_instruction(position, instruction, Some(warrior_id));
            }

            // Push initial task.
            let task = (starting_position + context.warrior.origin) % self.core.size();
            context.task_queue.push_if_not_full(task);
        }
    }

    fn determine_starting_positions(&self) -> Vec<usize> {
        match self.config.warrior_separation_strategy {
            WarriorSeparationStrategy::Equal => self.determine_starting_positions_equal(),
            WarriorSeparationStrategy::Random => self.determine_starting_positions_random(),
        }
    }

    fn determine_starting_positions_equal(&self) -> Vec<usize> {
        let num_warriors = self.warrior_contexts.len();

        (0..num_warriors)
            .map(|warrior_id| self.core.size() / num_warriors * warrior_id)
            .collect()
    }

    fn determine_starting_positions_random(&self) -> Vec<usize> {
        let num_warriors = self.warrior_contexts.len();

        let instruction_lengths = self
            .warrior_contexts
            .iter()
            .map(|context| context.warrior.instructions.len())
            .collect::<Vec<_>>();

        let separation_buckets = {
            let total_instructions = self
                .warrior_contexts
                .iter()
                .map(|context| context.warrior.instructions.len())
                .sum::<usize>();

            let mut buckets = vec![self.config.min_distance_between_warriors; num_warriors];
            let mut remaining_cells = self.core.size()
                - total_instructions
                - (self.config.min_distance_between_warriors * num_warriors);

            while remaining_cells != 0 {
                let bucket_id = rng::rand_range(0, num_warriors);
                buckets[bucket_id] += 1;
                remaining_cells -= 1;
            }

            buckets
        };

        let mut positions = Vec::with_capacity(num_warriors);
        let mut position = 0;

        for (instructions, separation) in instruction_lengths.iter().zip(separation_buckets.iter())
        {
            positions.push(position);
            position += instructions + separation;
        }

        rng::shuffle(&mut positions);
        positions
    }

    pub fn reset(&mut self, loading_next_game: bool) {
        self.core.reset();

        for context in &mut self.warrior_contexts {
            context.task_queue.clear();
        }
        self.load_warriors_to_core_and_initialize_task_queues();

        if loading_next_game {
            self.game_counter += 1;
        }

        self.turn_counter = 0;
        self.cycle_counter = 0;
        self.current_warrior_id = 0;
        self.game_over = false;
        self.winner = None;
    }

    pub fn step(&mut self) {
        if self.game_over {
            return;
        }

        Self::execute_task(
            self.current_warrior_id,
            &mut self.warrior_contexts[self.current_warrior_id].task_queue,
            &mut self.core,
        );

        self.cycle_counter += 1;

        if self.check_is_game_over() {
            self.set_game_over_and_determine_winner();
            return;
        }

        if let Some(warrior_id) = self.find_next_warrior_alive() {
            self.current_warrior_id = warrior_id;
        }
    }

    /// Example: In a 4-player game with warriors [0, 1, 2, 3], if `current_warrior_id` is 1,
    /// then we would try to find the next warrior alive at [2, 3], then advance turn counter, then try to find next warrior alive at [0, 1].
    fn find_next_warrior_alive(&mut self) -> Option<WarriorId> {
        // Check first pass - from next player to last player.
        if let Some(warrior_id) = (self.current_warrior_id + 1..self.warrior_contexts.len())
            .find(|&warrior_id| self.warrior_contexts[warrior_id].is_alive())
        {
            return Some(warrior_id);
        }

        // Advance the turn counter, and check if this ends the game.
        self.turn_counter += 1;
        if self.turn_counter >= self.config.turn_limit {
            self.set_game_over_and_determine_winner();
            return None;
        }

        // Check second pass - from first player to current player.
        (0..=self.current_warrior_id)
            .find(|&warrior_id| self.warrior_contexts[warrior_id].is_alive())
    }

    fn set_game_over_and_determine_winner(&mut self) {
        self.game_over = true;

        let warrior_ids_alive = (0..self.warrior_contexts.len())
            .filter(|&warrior_id| self.warrior_contexts[warrior_id].is_alive())
            .collect::<Vec<_>>();

        if warrior_ids_alive.len() == 1
            && let Some(&winner_id) = warrior_ids_alive.first()
        {
            self.winner = Some(winner_id);
            self.warrior_contexts[winner_id].num_wins += 1;
        }
    }

    fn check_is_game_over(&self) -> bool {
        // The game ends when `turn_counter` reaches maximum value.
        if self.turn_counter >= self.config.turn_limit {
            return true;
        }

        let num_warriors_alive = self
            .warrior_contexts
            .iter()
            .filter(|warrior| warrior.is_alive())
            .count();

        match self.warrior_contexts.len() {
            // In single-player mode, the game ends all players are dead.
            1 => num_warriors_alive == 0,
            // In multi-player mode, the game ends when only 1 players remain alive, or when all players are dead.
            _ => num_warriors_alive <= 1,
        }
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

    fn execute_by_opcode(
        instruction: &Instruction,
        address: Address,
        core: &mut Core,
        warrior_id: WarriorId,
    ) -> TaskOutcome {
        match instruction.operation.opcode {
            Opcode::DAT => exec_dat(instruction, address, core, warrior_id),

            Opcode::MOV => exec_mov(instruction, address, core, warrior_id),

            Opcode::ADD => exec_add(instruction, address, core, warrior_id),
            Opcode::SUB => exec_sub(instruction, address, core, warrior_id),
            Opcode::MUL => exec_mul(instruction, address, core, warrior_id),
            Opcode::DIV => exec_div(instruction, address, core, warrior_id),
            Opcode::MOD => exec_mod(instruction, address, core, warrior_id),

            Opcode::JMP => exec_jmp(instruction, address, core, warrior_id),
            Opcode::JMZ => exec_jmz(instruction, address, core, warrior_id),
            Opcode::JMN => exec_jmn(instruction, address, core, warrior_id),
            Opcode::DJN => exec_djn(instruction, address, core, warrior_id),

            Opcode::SPL => exec_spl(instruction, address, core, warrior_id),

            // `CMP` is just an alias for `SEQ`.
            Opcode::CMP | Opcode::SEQ => exec_seq(instruction, address, core, warrior_id),
            Opcode::SNE => exec_sne(instruction, address, core, warrior_id),
            Opcode::SLT => exec_slt(instruction, address, core, warrior_id),

            // `LDP` and `STP` are not implemented, so they are equivalent to `NOP`.
            Opcode::LDP | Opcode::STP | Opcode::NOP => {
                exec_nop(instruction, address, core, warrior_id)
            }
        }
    }
}
