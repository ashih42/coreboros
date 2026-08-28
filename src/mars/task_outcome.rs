use crate::mars::address::Address;

/// `TaskOutcome` represents all possible task outputs, given a task input.
pub enum TaskOutcome {
    Spawned {
        current_task: Address,
        new_task: Address,
    },
    Lived {
        current_task: Address,
    },
    Died,
}
