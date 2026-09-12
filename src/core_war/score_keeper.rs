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

    /// Get number of wins for `warrior_id`.
    pub fn get_wins(&self, warrior_id: WarriorId) -> usize {
        let index = warrior_id.as_index();

        #[allow(clippy::indexing_slicing, reason = "The index is valid.")]
        self.num_wins[index]
    }

    /// Increment number of wins for `warrior_id`.
    pub fn increment_wins(&mut self, warrior_id: WarriorId) {
        let index = warrior_id.as_index();

        #[allow(clippy::indexing_slicing, reason = "The index is valid.")]
        #[allow(
            clippy::arithmetic_side_effects,
            reason = "The number of wins is a small number."
        )]
        {
            self.num_wins[index] += 1;
        }
    }
}
