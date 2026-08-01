/// See ICWS 94 Standard, Section 4.2 <https://corewar.co.uk/standards/icws94.htm#4.2>
pub struct RuntimeConfig {
    pub core_size: usize,
    pub cycles_before_tie: u32,
    pub core_initialization_strategy: CoreInitializationStrategy,
    pub instruction_limit: usize,
    pub max_number_of_tasks: usize,
    pub minimum_separation: usize,
    pub read_distance: usize,
    pub warrior_separation_strategy: WarriorSeparationStrategy,
    pub num_warriors: usize,
    pub write_distance: usize,
}

impl RuntimeConfig {
    #[must_use]
    pub const fn new_icws86() -> Self {
        Self {
            core_size: 8192,
            cycles_before_tie: 100_000,
            core_initialization_strategy: CoreInitializationStrategy::FillDat00,
            instruction_limit: 300,
            max_number_of_tasks: 64,
            minimum_separation: 300,
            read_distance: 8192,
            warrior_separation_strategy: WarriorSeparationStrategy::Random,
            num_warriors: 2,
            write_distance: 8192,
        }
    }

    #[must_use]
    pub const fn new_koth() -> Self {
        Self {
            core_size: 8_000,
            cycles_before_tie: 80_000,
            core_initialization_strategy: CoreInitializationStrategy::FillDat00,
            instruction_limit: 100,
            max_number_of_tasks: 8_000,
            minimum_separation: 100,
            read_distance: 8000,
            warrior_separation_strategy: WarriorSeparationStrategy::Random,
            num_warriors: 2,
            write_distance: 8_000,
        }
    }

    #[must_use]
    pub const fn new_small() -> Self {
        Self {
            core_size: 400,
            cycles_before_tie: 4000,
            core_initialization_strategy: CoreInitializationStrategy::FillDat00,
            instruction_limit: 100,
            max_number_of_tasks: 4,
            minimum_separation: 40,
            read_distance: 400,
            warrior_separation_strategy: WarriorSeparationStrategy::Equal,
            num_warriors: 2,
            write_distance: 400,
        }
    }
}

#[derive(Clone, Copy)]
pub enum CoreInitializationStrategy {
    FillDat00,
    None,
    Random,
}

pub enum WarriorSeparationStrategy {
    Min,
    Random,
    Equal,
}
