use crate::{
    core_war::{
        config::{Config, warrior_separation_strategy::WarriorSeparationStrategy},
        mars::core::Core,
    },
    rng,
    warrior::{Warrior, warrior_id::WarriorId},
};

/// `CorePlacementPlanner` is responsible for determining where to copy each warrior's instructions to the core.
pub struct CorePlacementPlanner {
    warrior_separation_strategy: WarriorSeparationStrategy,
    min_distance_between_warriors: usize,
}

impl CorePlacementPlanner {
    pub const fn new(config: &Config) -> Self {
        Self {
            warrior_separation_strategy: config.warrior_separation_strategy,
            min_distance_between_warriors: config.min_distance_between_warriors,
        }
    }

    /// Return the addresses to load each warrior's instructions to the core.
    pub fn determine_starting_addresses(&self, core: &Core, warriors: &[Warrior]) -> Vec<usize> {
        match self.warrior_separation_strategy {
            WarriorSeparationStrategy::Equal => {
                Self::determine_starting_addresses_equal(core, warriors)
            }
            WarriorSeparationStrategy::Random => {
                self.determine_starting_addresses_random(core, warriors)
            }
        }
    }

    /// Determine thestarting addresses under the `Equal` warrior separation strategy.
    fn determine_starting_addresses_equal(core: &Core, warriors: &[Warrior]) -> Vec<usize> {
        let core_size = core.get_size();

        #[allow(clippy::arithmetic_side_effects, reason = "These numbers are small.")]
        (0..warriors.len())
            .map(|warrior_id| core_size / warriors.len() * warrior_id)
            .collect()
    }

    /// Determine the starting addresses under the `Random` warrior separation strategy.
    /// Note: This also shuffles the warrior order for randomization, and then sort the result at the end,
    /// so the returned Vec contains the assigned addresses for warriors 0, 1, 2, etc.
    #[allow(clippy::indexing_slicing, reason = "The index is valid.")]
    #[allow(clippy::arithmetic_side_effects, reason = "The numbers are small.")]
    fn determine_starting_addresses_random(&self, core: &Core, warriors: &[Warrior]) -> Vec<usize> {
        let num_warriors = warriors.len();

        let instruction_lengths = warriors
            .iter()
            .map(|warrior| warrior.instructions.len())
            .collect::<Vec<_>>();

        let separation_buckets = self.initialize_buckets(core, warriors);

        let mut assignments = Vec::with_capacity(num_warriors);
        let mut address = 0;

        // TODO: Create a get_random_warrior_ids() util function.
        let warrior_ids = {
            let mut warrior_ids = (0..num_warriors).collect::<Vec<_>>();
            rng::shuffle(&mut warrior_ids);
            warrior_ids
        };

        for (&warrior_id, &separation) in warrior_ids.iter().zip(separation_buckets.iter()) {
            assignments.push(WarriorToAddressAssignment::new(warrior_id, address));
            address += instruction_lengths[warrior_id] + separation;
        }

        assignments.sort_by_key(|assignment| assignment.warrior_id);

        assignments
            .iter()
            .map(|assignment| assignment.address)
            .collect()
    }

    /// Return "buckets", which represents the blocks of empty cells separating different warriors' instructions.
    /// Example: buckets [10, 20] means there are 10 empty cells between warriors 0 and 1; and 20 empty cells between warriors 1 and 0.
    #[allow(clippy::indexing_slicing, reason = "The index is valid.")]
    #[allow(clippy::arithmetic_side_effects, reason = "The numbers are small.")]
    fn initialize_buckets(&self, core: &Core, warriors: &[Warrior]) -> Vec<usize> {
        let core_size = core.get_size();
        let num_warriors = warriors.len();

        let total_instructions = warriors
            .iter()
            .map(|warrior| warrior.instructions.len())
            .sum::<usize>();

        let mut buckets = vec![self.min_distance_between_warriors; num_warriors];

        let mut remaining_cells =
            core_size - total_instructions - (self.min_distance_between_warriors * num_warriors);

        while remaining_cells != 0 {
            let bucket_id = rng::rand_range(0, num_warriors);
            buckets[bucket_id] += 1;
            remaining_cells -= 1;
        }

        buckets
    }
}

struct WarriorToAddressAssignment {
    warrior_id: WarriorId,
    address: usize,
}

impl WarriorToAddressAssignment {
    pub const fn new(warrior_id: WarriorId, address: usize) -> Self {
        Self {
            warrior_id,
            address,
        }
    }
}
