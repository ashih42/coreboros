use crate::mars::address::Address;

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
