use std::collections::VecDeque;

use crate::mars::address::Address;

/// `TaskQueue` implements a FIFO queue container for "tasks", which are simply addresses on the core.
#[derive(Debug, Clone)]
pub struct TaskQueue {
    tasks: VecDeque<Address>,
}

impl TaskQueue {
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            tasks: VecDeque::with_capacity(capacity),
        }
    }

    pub fn push_if_not_full(&mut self, address: Address) {
        if self.tasks.len() < self.tasks.capacity() {
            self.tasks.push_back(address);
        }
    }

    pub fn peek(&self) -> Option<Address> {
        self.tasks.front().copied()
    }

    pub fn pop(&mut self) -> Option<Address> {
        self.tasks.pop_front()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Address> {
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

    pub fn contains(&self, address: &Address) -> bool {
        self.tasks.contains(address)
    }

    pub fn clear(&mut self) {
        self.tasks.clear();
    }
}
