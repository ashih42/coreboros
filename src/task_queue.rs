use std::collections::VecDeque;

use crate::task::Task;

pub struct TaskQueue {
    tasks: VecDeque<Task>,
}

impl TaskQueue {
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            tasks: VecDeque::with_capacity(capacity),
        }
    }

    /// Push a task only if capacity is not full.
    pub fn try_push(&mut self, task: Task) {
        if self.tasks.len() < self.tasks.capacity() {
            self.tasks.push_back(task);
        }
    }

    pub fn pop(&mut self) -> Option<Task> {
        self.tasks.pop_front()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}
