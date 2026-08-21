use crate::mars::config::{
    core_dimension::CoreDimension, core_initialization_strategy::CoreInitializationStrategy,
};

pub mod core_dimension;
pub mod core_initialization_strategy;
pub mod warrior_separation_strategy;

pub struct Config {
    pub core_dimension: CoreDimension,

    pub core_initialization_strategy: CoreInitializationStrategy,
    pub cycles_before_tie: u32,
    pub max_number_of_tasks: usize,
    // TODO: warrior_separation_strategy: WarriorSeparationStrategy
}
