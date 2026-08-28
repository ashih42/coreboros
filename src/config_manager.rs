use crate::{
    mars::config::{
        Config, core_dimension::CoreDimension,
        core_initialization_strategy::CoreInitializationStrategy,
        warrior_separation_strategy::WarriorSeparationStrategy,
    },
    warrior_queue::WarriorQueue,
};

/// `ConfigManager` contains the list of available values and the currently selected value for each feature,
/// used in dropdown selector widgets.
pub struct ConfigManager {
    pub selected_core_dimension: CoreDimension,
    pub available_core_dimensions: Box<[CoreDimension]>,

    pub selected_core_initialization_strategy: CoreInitializationStrategy,
    pub available_core_initialization_strategies: Box<[CoreInitializationStrategy]>,

    pub selected_task_queue_capacity: usize,
    pub available_task_queue_capacities: Box<[usize]>,

    pub selected_turn_limit: usize,
    pub available_turn_limits: Box<[usize]>,

    pub selected_warrior_separation_strategy: WarriorSeparationStrategy,
    pub available_warrior_separation_strategies: Box<[WarriorSeparationStrategy]>,

    pub selected_min_distance_between_warriors: usize,
    pub available_min_distance_between_warriors: Box<[usize]>,
}

impl Default for ConfigManager {
    fn default() -> Self {
        let selected_core_dimension = CoreDimension::Nano;
        let available_core_dimensions = CoreDimension::list_all_values();

        let selected_core_initialization_strategy = CoreInitializationStrategy::FillDat00;
        let available_core_initialization_strategies =
            CoreInitializationStrategy::list_all_values();

        let selected_task_queue_capacity = 64;
        let available_task_queue_capacities = Box::new([1, 4, 16, 64, 128, 256]);

        let selected_turn_limit = 400;
        let available_turn_limits = Box::new([
            40, 80, 200, 400, 800, 2_000, 4_000, 8_000, 20_000, 40_000, 80_000,
        ]);

        let selected_warrior_separation_strategy = WarriorSeparationStrategy::Random;
        let available_warrior_separation_strategies = WarriorSeparationStrategy::list_all_values();

        let selected_min_distance_between_warriors = 10;
        let available_min_distance_between_warriors = Box::new([0, 10, 20, 30, 40]);

        Self {
            selected_core_dimension,
            available_core_dimensions,
            selected_core_initialization_strategy,
            available_core_initialization_strategies,
            selected_task_queue_capacity,
            available_task_queue_capacities,
            selected_turn_limit,
            available_turn_limits,
            selected_warrior_separation_strategy,
            available_warrior_separation_strategies,
            selected_min_distance_between_warriors,
            available_min_distance_between_warriors,
        }
    }
}

impl ConfigManager {
    /// Return a `Config` with the currently selected values.
    pub const fn get_config(&self) -> Config {
        Config {
            core_dimension: self.selected_core_dimension,
            core_initialization_strategy: self.selected_core_initialization_strategy,
            task_queue_capacity: self.selected_task_queue_capacity,
            turn_limit: self.selected_turn_limit,
            warrior_separation_strategy: self.selected_warrior_separation_strategy,
            min_distance_between_warriors: self.selected_min_distance_between_warriors,
        }
    }

    /// Check if the core has enough space to fit all warriors in `warrior_queue`,
    /// while considering constraints in the currently selected config values
    /// `selected_core_dimension`, `selected_warrior_separation_strategy`, and `selected_min_distance_between_warriors`.
    #[allow(
        clippy::arithmetic_side_effects,
        reason = "These numbers are small because user input is limited."
    )]
    pub fn validate_entry(&self, warrior_queue: &WarriorQueue) -> Result<(), String> {
        let total_instructions = warrior_queue
            .iter()
            .map(|warrior| warrior.instructions.len())
            .sum::<usize>();

        let total_separation_distance = match warrior_queue.len() {
            1 => 0,
            _ => match self.selected_warrior_separation_strategy {
                WarriorSeparationStrategy::Equal => 0,
                WarriorSeparationStrategy::Random => {
                    self.selected_min_distance_between_warriors * warrior_queue.len()
                }
            },
        };

        let required_size = total_instructions + total_separation_distance;
        let available_size = self.selected_core_dimension.as_size();

        if required_size <= available_size {
            Ok(())
        } else {
            Err(indoc::formatdoc!(
                "Error: Not enough space in core for warriors.
                
                These {} warriors together have {total_instructions} instructions,
                and we need at least {total_separation_distance} cells between them,
                so the required core size is at least {required_size} cells.

                But the current core only has {available_size} cells.",
                warrior_queue.len()
            ))
        }
    }
}
