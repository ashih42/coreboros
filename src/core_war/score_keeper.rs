use crate::warrior::warrior_id::WarriorId;

pub struct ScoreKeeper {
    num_wins: Box<[usize]>,
}

impl ScoreKeeper {
    pub fn new(num_warriors: usize) -> Self {
        Self {
            num_wins: vec![0; num_warriors].into_boxed_slice(),
        }
    }

    pub fn get_wins(&self, warrior_id: WarriorId) -> usize {
        #[allow(clippy::indexing_slicing, reason = "The index is valid.")]
        self.num_wins[warrior_id]
    }

    #[allow(clippy::indexing_slicing, reason = "The index is valid.")]
    #[allow(
        clippy::arithmetic_side_effects,
        reason = "The number of wins are small numbers."
    )]
    pub fn increment_wins(&mut self, warrior_id: WarriorId) {
        self.num_wins[warrior_id] += 1;
    }
}
