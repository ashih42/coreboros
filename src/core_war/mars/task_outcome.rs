use crate::core_war::core_number::CoreNumber;

/// `TaskOutcome` represents all possible task outputs, given a task input.
pub enum TaskOutcome {
    Spawned {
        current_task: CoreNumber,
        new_task: CoreNumber,
    },
    Lived {
        current_task: CoreNumber,
    },
    Died,
}
