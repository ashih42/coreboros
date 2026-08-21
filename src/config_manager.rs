use crate::mars::config::{
    Config, core_dimension::CoreDimension, core_initialization_strategy::CoreInitializationStrategy,
};

pub struct ConfigManager {
    pub selected_core_dimension: CoreDimension,
    pub available_core_dimensions: Vec<CoreDimension>,

    pub selected_core_initialization_strategy: CoreInitializationStrategy,
    pub available_core_initialization_strategies: Vec<CoreInitializationStrategy>,
}

impl Default for ConfigManager {
    fn default() -> Self {
        let selected_core_dimension = CoreDimension::Nano;
        let available_core_dimensions = CoreDimension::list_all_values();

        let selected_core_initialization_strategy = CoreInitializationStrategy::FillDat00;
        let available_core_initialization_strategies =
            CoreInitializationStrategy::list_all_values();

        Self {
            selected_core_dimension,
            available_core_dimensions,
            selected_core_initialization_strategy,
            available_core_initialization_strategies,
        }
    }
}

impl ConfigManager {
    pub fn get_config(&self) -> Config {
        Config {
            core_dimension: self.selected_core_dimension,
            core_initialization_strategy: self.selected_core_initialization_strategy,

            cycles_before_tie: 8_000,
            max_number_of_tasks: 64,
        }
    }
}
