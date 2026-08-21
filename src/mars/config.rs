use crate::mars::config::{
    core_dimension::CoreDimension, core_initialization_strategy::CoreInitializationStrategy,
};

pub mod core_dimension;
pub mod core_initialization_strategy;
pub mod warrior_separation_strategy;

pub struct Config {
    pub core_dimension: CoreDimension,
    pub core_initialization_strategy: CoreInitializationStrategy,
    pub task_queue_capacity: usize,
    //
    pub turn_limit: usize,
    // TODO: warrior_separation_strategy: WarriorSeparationStrategy
}
