use crate::mars::config::{
    Config, core_dimension::CoreDimension, core_initialization_strategy::CoreInitializationStrategy,
};

pub struct ConfigManager {
    pub selected_core_dimension: CoreDimension,
    pub available_core_dimensions: Vec<CoreDimension>,

    pub selected_core_initialization_strategy: CoreInitializationStrategy,
    pub available_core_initialization_strategies: Vec<CoreInitializationStrategy>,

    pub selected_task_queue_capacity: usize,
    pub available_task_queue_capacities: Vec<usize>,
}

impl Default for ConfigManager {
    fn default() -> Self {
        let selected_core_dimension = CoreDimension::Nano;
        let available_core_dimensions = CoreDimension::list_all_values();

        let selected_core_initialization_strategy = CoreInitializationStrategy::FillDat00;
        let available_core_initialization_strategies =
            CoreInitializationStrategy::list_all_values();

        let selected_task_queue_capacity = 64;
        let available_task_queue_capacities = vec![1, 4, 16, 64, 128, 256];

        Self {
            selected_core_dimension,
            available_core_dimensions,
            selected_core_initialization_strategy,
            available_core_initialization_strategies,
            selected_task_queue_capacity,
            available_task_queue_capacities,
        }
    }
}

impl ConfigManager {
    pub fn get_config(&self) -> Config {
        Config {
            core_dimension: self.selected_core_dimension,
            core_initialization_strategy: self.selected_core_initialization_strategy,
            task_queue_capacity: self.selected_task_queue_capacity,

            cycles_before_tie: 8_000,
        }
    }
}
