use crate::mars::config::{
    core_dimension::CoreDimension, core_initialization_strategy::CoreInitializationStrategy,
    warrior_separation_strategy::WarriorSeparationStrategy,
};

pub mod core_dimension;
pub mod core_initialization_strategy;
pub mod warrior_separation_strategy;

/// `Config` contains user-selected values that may affect various operations in `Mars`.
pub struct Config {
    pub core_dimension: CoreDimension,
    pub core_initialization_strategy: CoreInitializationStrategy,
    pub task_queue_capacity: usize,
    pub turn_limit: usize,
    pub warrior_separation_strategy: WarriorSeparationStrategy,
    pub min_distance_between_warriors: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            core_dimension: CoreDimension::Nano,
            core_initialization_strategy: CoreInitializationStrategy::FillDat00,
            task_queue_capacity: 64,
            turn_limit: 2000,
            warrior_separation_strategy: WarriorSeparationStrategy::Equal,
            min_distance_between_warriors: 10,
        }
    }
}
