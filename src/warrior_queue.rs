use crate::warrior::Warrior;

/// `WarriorQueue` is an ordered collection of `Warrior` instances ready for gameplay.
pub struct WarriorQueue {
    warriors: Vec<Warrior>,
}

/// Currently, `WarriorQueue` allows at most 8 `Warriors` because there is only a small number
/// of easily distinguishable colors available for rendering.
pub const MAX_CAPACITY: usize = 8;

impl Default for WarriorQueue {
    fn default() -> Self {
        Self {
            warriors: Vec::with_capacity(MAX_CAPACITY),
        }
    }
}

impl<'a, I> From<I> for WarriorQueue
where
    I: Iterator<Item = &'a Warrior>,
{
    /// Convert from an iterator of `&Warrior` to `WarriorQueue`.
    fn from(iter: I) -> Self {
        let mut warriors = Vec::with_capacity(MAX_CAPACITY);
        warriors.extend(iter.take(MAX_CAPACITY).cloned());

        Self { warriors }
    }
}

impl From<WarriorQueue> for Vec<Warrior> {
    /// Convert from `WarriorQueue` to `Vec<Warrior>`.
    fn from(warrior_queue: WarriorQueue) -> Self {
        warrior_queue.warriors
    }
}

impl WarriorQueue {
    pub const fn get_capacity(&self) -> usize {
        self.warriors.capacity()
    }

    pub fn is_ready(&self) -> bool {
        (1..=self.warriors.capacity()).contains(&self.warriors.len())
    }

    pub const fn is_full(&self) -> bool {
        self.warriors.len() == self.warriors.capacity()
    }

    pub const fn len(&self) -> usize {
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

    /// Move the entity at `index` up one position,
    /// effectively swapping the entities at `index` and `index - 1`.
    /// Note: `index` must be valid.
    #[allow(
        clippy::arithmetic_side_effects,
        reason = "The expression is guaranteed to be valid."
    )]
    pub fn move_up(&mut self, index: usize) {
        if index >= 1 {
            self.warriors.swap(index, index - 1);
        }
    }

    /// Move the entity at `index` down one position,
    /// effectively swapping the entities at `index` and `index + 1`.
    /// Note: `index` must be valid.
    #[allow(
        clippy::arithmetic_side_effects,
        reason = "The expression is guaranteed to be valid."
    )]
    pub fn move_down(&mut self, index: usize) {
        if index + 1 < self.warriors.len() {
            self.warriors.swap(index, index + 1);
        }
    }
}
