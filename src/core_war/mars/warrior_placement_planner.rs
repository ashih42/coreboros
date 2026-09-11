use crate::{
    core_war::{
        config::{Config, warrior_separation_strategy::WarriorSeparationStrategy},
        mars::core::Core,
    },
    rng,
    warrior::{Warrior, warrior_id::WarriorId},
};

/// `WarriorPlacementPlanner` is responsible for determining the addresses on the core
/// to load each warrior's instructions when a game is initialized.
pub struct WarriorPlacementPlanner {
    warrior_separation_strategy: WarriorSeparationStrategy,
    min_distance_between_warriors: usize,
}

/// `Placement` is an assignment of a `WarriorId` for his instructions to be loaded to `address`.
/// This is used internally when the warrior order is also randomized.
struct Placement {
    warrior_id: WarriorId,
    address: usize,
}

impl WarriorPlacementPlanner {
    pub const fn new(config: &Config) -> Self {
        Self {
            warrior_separation_strategy: config.warrior_separation_strategy,
            min_distance_between_warriors: config.min_distance_between_warriors,
        }
    }

    /// Return the addresses to load each warrior's instructions to the core, in default warrior order.
    /// Example: [0, 40] means to load warrior 0 at address 0, and warrior 1 at address 40.
    pub fn determine_placements(&self, core: &Core, warriors: &[Warrior]) -> Box<[usize]> {
        match self.warrior_separation_strategy {
            WarriorSeparationStrategy::Equal => Self::determine_placements_equal(core, warriors),
            WarriorSeparationStrategy::Random => self.determine_placements_random(core, warriors),
        }
    }

    /// Determine placements under the `Equal` warrior separation strategy.
    fn determine_placements_equal(core: &Core, warriors: &[Warrior]) -> Box<[usize]> {
        let core_size = core.get_size();

        #[allow(clippy::arithmetic_side_effects, reason = "These numbers are small.")]
        (0..warriors.len())
            .map(|warrior_id| core_size / warriors.len() * warrior_id)
            .collect()
    }

    /// Determine placements under the `Random` warrior separation strategy.
    /// Note: This also randomizes the warrior order, so the result might look like
    /// [40, 0], i.e. to load warrior 0 at address 40, and warrior 1 at address 0.
    fn determine_placements_random(&self, core: &Core, warriors: &[Warrior]) -> Box<[usize]> {
        let num_warriors = warriors.len();

        let mut placements = Vec::with_capacity(num_warriors);
        let mut address = 0;

        for (&warrior_id, separator) in Self::get_warrior_ids_in_random_order(num_warriors)
            .iter()
            .zip(self.generate_random_separators(core, warriors))
        {
            placements.push(Placement {
                warrior_id,
                address,
            });

            #[allow(clippy::indexing_slicing, reason = "The index is valid 👌")]
            let instruction_length = warriors[warrior_id.as_index()].instructions.len();

            #[allow(clippy::arithmetic_side_effects, reason = "These operations are safe.")]
            {
                address += instruction_length + separator;
            }
        }

        placements.sort_by_key(|assignment| assignment.warrior_id);

        placements
            .iter()
            .map(|assignment| assignment.address)
            .collect()
    }

    /// Return a randomly shuffled list of all `WarriorId` values.
    fn get_warrior_ids_in_random_order(num_warriors: usize) -> Box<[WarriorId]> {
        let mut warrior_ids = WarriorId::list_all_warrior_ids(num_warriors).collect::<Vec<_>>();
        rng::shuffle(&mut warrior_ids);

        warrior_ids.into_boxed_slice()
    }

    /// Return "separators", which are the sizes of the blocks of empty cells between different warriors' instructions.
    /// Example: separators [10, 20] means there are 10 empty cells between warriors 0 and 1, and 20 empty cells between warriors 1 and 0.
    #[allow(clippy::indexing_slicing, reason = "The index is valid.")]
    #[allow(clippy::arithmetic_side_effects, reason = "The numbers are small.")]
    fn generate_random_separators(&self, core: &Core, warriors: &[Warrior]) -> Box<[usize]> {
        let core_size = core.get_size();
        let num_warriors = warriors.len();

        let total_instructions = warriors
            .iter()
            .map(|warrior| warrior.instructions.len())
            .sum::<usize>();

        let mut separators = vec![self.min_distance_between_warriors; num_warriors];

        let mut remaining_cells =
            core_size - total_instructions - (self.min_distance_between_warriors * num_warriors);

        while remaining_cells != 0 {
            let index = rng::rand_range(0, num_warriors);
            separators[index] += 1;
            remaining_cells -= 1;
        }

        separators.into_boxed_slice()
    }
}
