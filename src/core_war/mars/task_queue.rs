use std::collections::VecDeque;

use crate::core_war::core_number::CoreNumber;

/// `TaskQueue` is a FIFO queue for "tasks", which are simply addresses in the core.
#[derive(Debug)]
pub struct TaskQueue {
    tasks: VecDeque<CoreNumber>,
}

impl TaskQueue {
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            tasks: VecDeque::with_capacity(capacity),
        }
    }

    pub fn get_capacity(&self) -> usize {
        self.tasks.capacity()
    }

    pub fn push_if_not_full(&mut self, task: CoreNumber) {
        if self.tasks.len() < self.tasks.capacity() {
            self.tasks.push_back(task);
        }
    }

    pub fn peek(&self) -> Option<CoreNumber> {
        self.tasks.front().copied()
    }

    pub fn pop(&mut self) -> Option<CoreNumber> {
        self.tasks.pop_front()
    }

    pub fn iter(&self) -> impl Iterator<Item = &CoreNumber> {
        self.tasks.iter()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.tasks.len() == self.tasks.capacity()
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn contains(&self, task: CoreNumber) -> bool {
        self.tasks.contains(&task)
    }

    pub fn clear(&mut self) {
        self.tasks.clear();
    }
}
