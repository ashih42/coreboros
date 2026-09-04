use crate::warrior::Warrior;

/// `WarriorQueue` is an ordered collection of `Warrior` instances ready for gameplay.
pub struct WarriorQueue {
    warriors: Vec<Warrior>,
}

/// Currently, `WarriorQueue` allows at most 8 `Warrior` in the queue because there are only
/// these many easily distinguishable colors available for rendering.
const MAX_CAPACITY: usize = 8;

impl Default for WarriorQueue {
    fn default() -> Self {
        Self {
            warriors: Vec::with_capacity(MAX_CAPACITY),
        }
    }
}

impl From<Box<[Warrior]>> for WarriorQueue {
    /// Convert from a `Box<[Warrior]>` to `WarriorQueue`.
    fn from(input: Box<[Warrior]>) -> Self {
        let mut warriors = Vec::with_capacity(MAX_CAPACITY);
        warriors.extend(input.into_iter().take(MAX_CAPACITY));

        Self { warriors }
    }
}

impl WarriorQueue {
    pub fn into_boxed_warriors(self) -> Box<[Warrior]> {
        self.warriors.into_boxed_slice()
    }

    pub const fn get_capacity(&self) -> usize {
        self.warriors.capacity()
    }

    /// Return a bool indicating if `WarriorQueue` contains a valid number of `Warrior` to enter the arena.
    /// A valid number of `Warrior` is in range `[1, MAX_CAPACIT]`.
    pub fn is_ready_for_arena(&self) -> bool {
        (1..=MAX_CAPACITY).contains(&self.warriors.len())
    }

    pub const fn is_full(&self) -> bool {
        self.warriors.len() == MAX_CAPACITY
    }

    pub const fn len(&self) -> usize {
        self.warriors.len()
    }

    pub const fn as_slice(&self) -> &[Warrior] {
        self.warriors.as_slice()
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
