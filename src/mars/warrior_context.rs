use crate::{mars::task_queue::TaskQueue, warrior::Warrior};

pub struct WarriorContext {
    pub warrior: Warrior,
    pub task_queue: TaskQueue,
    pub num_wins: usize,
}

impl WarriorContext {
    pub const fn new(warrior: Warrior, task_queue: TaskQueue) -> Self {
        Self {
            warrior,
            task_queue,
            num_wins: 0,
        }
    }

    pub fn is_alive(&self) -> bool {
        !self.task_queue.is_empty()
    }
}
