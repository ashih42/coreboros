use crate::{
    core_war::{config::Config, mars::Mars, score_keeper::ScoreKeeper},
    warrior::{Warrior, warrior_id::WarriorId},
};

pub mod address;
pub mod config;
pub mod mars;

mod score_keeper;

/// `CoreWar` acts as the Game Manager, keeps track of whose turn it is, and checks for game-over conditions.
/// The game logic of executing instructions is delegated to `Mars`.
pub struct CoreWar {
    pub warriors: Box<[Warrior]>,
    pub config: Config,
    pub mars: Mars,
    pub score_keeper: ScoreKeeper,
    pub game_counter: usize,
    pub turn_counter: usize,
    pub cycle_counter: usize,
    pub current_warrior_id: WarriorId,
    pub game_over: bool,
    pub winner: Option<WarriorId>,
}

impl CoreWar {
    pub fn new(warriors: Box<[Warrior]>, config: Config) -> Self {
        let mars = Mars::new(&warriors, &config);
        let score_keeper = ScoreKeeper::new(warriors.len());

        Self {
            warriors,
            config,
            mars,
            score_keeper,
            game_counter: 0,
            turn_counter: 0,
            cycle_counter: 0,
            current_warrior_id: WarriorId(0),
            game_over: false,
            winner: None,
        }
    }

    /// Reset `mars` and game stats for a new game.
    pub fn reset(&mut self, incrementing_game_counter: bool) {
        self.mars.reset(&self.warriors);

        #[allow(clippy::arithmetic_side_effects, reason = "`game_counter` is small.")]
        if incrementing_game_counter {
            self.game_counter += 1;
        }

        self.turn_counter = 0;
        self.cycle_counter = 0;
        self.current_warrior_id = WarriorId(0);
        self.game_over = false;
        self.winner = None;
    }

    /// Execute one instruction.
    #[allow(clippy::indexing_slicing, reason = "The index is valid.")]
    #[allow(clippy::arithmetic_side_effects, reason = "`cycle_counter` is small.")]
    pub fn step(&mut self) {
        if self.game_over {
            return;
        }

        self.mars.step(self.current_warrior_id);

        self.cycle_counter += 1;

        if self.check_is_game_over() {
            self.set_game_over_and_determine_winner();
            return;
        }

        if let Some(warrior_id) = self.find_next_warrior_alive() {
            self.current_warrior_id = warrior_id;
        }
    }

    /// Check if it is game over from:
    /// - reaching the final turn.
    /// - observing enough warriors have died.
    fn check_is_game_over(&self) -> bool {
        // The game ends when `turn_counter` reaches maximum value.
        if self.turn_counter >= self.config.turn_limit {
            return true;
        }

        let num_warriors_alive = WarriorId::list_all_warrior_ids(self.warriors.len())
            .filter(|&warrior_id| self.is_warrior_alive(warrior_id))
            .count();

        match self.warriors.len() {
            // In single-player mode, the game ends all players are dead.
            1 => num_warriors_alive == 0,
            // In multi-player mode, the game ends when only 1 players remain alive, or when all players are dead.
            _ => num_warriors_alive <= 1,
        }
    }

    /// Find the next warrior still alive to execute his instruction next.
    ///
    /// Example: In a 4-player game with warriors [0, 1, 2, 3], if `current_warrior_id` is 1,
    /// then we would try to find the next warrior alive at [2, 3], then advance turn counter, then try to find next warrior alive at [0, 1].
    #[allow(clippy::indexing_slicing, reason = "The index is valid.")]
    #[allow(clippy::arithmetic_side_effects, reason = "The numbers are small.")]
    fn find_next_warrior_alive(&mut self) -> Option<WarriorId> {
        // Check first pass - from next player to last player.
        if let Some(warrior_id) = ((self.current_warrior_id.0 + 1)..self.warriors.len())
            .map(WarriorId)
            .find(|&warrior_id| self.is_warrior_alive(warrior_id))
        {
            return Some(warrior_id);
        }

        // Advance the turn counter, and check if this ends the game.
        self.turn_counter += 1;
        if self.turn_counter >= self.config.turn_limit {
            self.set_game_over_and_determine_winner();
            return None;
        }

        // Check second pass - from first player to current player.
        (0..=self.current_warrior_id.0)
            .map(WarriorId)
            .find(|&warrior_id| self.is_warrior_alive(warrior_id))
    }

    /// Set `game_over` flag and determine if there is a winner.
    fn set_game_over_and_determine_winner(&mut self) {
        self.game_over = true;

        let warrior_ids_alive = WarriorId::list_all_warrior_ids(self.warriors.len())
            .filter(|&warrior_id| self.is_warrior_alive(warrior_id))
            .collect::<Vec<_>>();

        if warrior_ids_alive.len() == 1
            && let Some(&winner_id) = warrior_ids_alive.first()
        {
            self.winner = Some(winner_id);
            self.score_keeper.increment_wins(winner_id);
        }
    }

    /// Check if a specific warrior is still alive.
    /// A warrior is alive if he has at least one task to execute.
    pub fn is_warrior_alive(&self, warrior_id: WarriorId) -> bool {
        #[allow(clippy::indexing_slicing, reason = "This index is valid 👌")]
        let task_queue = &self.mars.task_queues[warrior_id.0];

        !task_queue.is_empty()
    }
}
