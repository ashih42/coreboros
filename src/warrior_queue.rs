use crate::warrior::Warrior;

pub struct WarriorQueue {
    warriors: Vec<Warrior>,
}

pub const MAX_CAPACITY: usize = 4;

impl Default for WarriorQueue {
    fn default() -> Self {
        Self {
            warriors: Vec::with_capacity(MAX_CAPACITY),
        }
    }
}

/// Convert from an iterator of `&Warrior` to `WarriorQueue`.
impl<'a, I> From<I> for WarriorQueue
where
    I: Iterator<Item = &'a Warrior>,
{
    fn from(iter: I) -> Self {
        let mut warriors = Vec::with_capacity(MAX_CAPACITY);
        warriors.extend(iter.take(MAX_CAPACITY).cloned());

        Self { warriors }
    }
}

/// Convert from `WarriorQueue` to `Vec<Warrior>`.
impl From<WarriorQueue> for Vec<Warrior> {
    fn from(warrior_queue: WarriorQueue) -> Self {
        warrior_queue.warriors
    }
}

impl WarriorQueue {
    pub fn get_capacity(&self) -> usize {
        self.warriors.capacity()
    }

    pub fn is_ready(&self) -> bool {
        (1..=self.warriors.capacity()).contains(&self.warriors.len())
    }

    pub fn is_full(&self) -> bool {
        self.warriors.len() == self.warriors.capacity()
    }

    pub fn len(&self) -> usize {
        self.warriors.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Warrior> {
        self.warriors.iter()
    }

    pub fn get(&self, index: usize) -> Option<&Warrior> {
        self.warriors.get(index)
    }

    pub fn remove(&mut self, index: usize) {
        self.warriors.remove(index);
    }

    pub fn push_if_not_full(&mut self, warrior: Warrior) {
        if !self.is_full() {
            self.warriors.push(warrior);
        }
    }

    /// Invariant: `index` is valid.
    pub fn move_up(&mut self, index: usize) {
        if index >= 1 {
            self.warriors.swap(index, index - 1);
        }
    }

    /// Invariant: `index` is valid.
    pub fn move_down(&mut self, index: usize) {
        if index + 1 < self.warriors.len() {
            self.warriors.swap(index, index + 1);
        }
    }
}
