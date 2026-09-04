use crate::warrior::Warrior;

/// `WarriorVault` is a collection of `Warrior` stored instances that may be edited and loaded for gameplay.
/// Note: `warrior.metadata.name` is used as a primary key to in `WarriorVault`.
pub struct WarriorVault {
    warriors: Vec<Warrior>,
}

impl Default for WarriorVault {
    fn default() -> Self {
        let warriors = vec![
            Warrior::dwarf(),
            Warrior::dwarf_2(),
            Warrior::imp(),
            Warrior::imp_factory(),
            Warrior::nop(),
            Warrior::nop_20(),
            Warrior::looping_paper(),
            Warrior::blur_scanner(),
            Warrior::transposition_stone(),
            Warrior::self_bombing_stone(),
            Warrior::self_vamping_vampire(),
        ];

        let mut vault = Self { warriors };
        vault.sort_warriors_by_name();

        vault
    }
}

impl WarriorVault {
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

    /// Save the given `warrior`, using `warrior.metadata.name` as a primary key in the `WarriorVault`.
    pub fn save_warrior(&mut self, warrior: &Warrior) {
        let name = &warrior.metadata.name;

        match self
            .warriors
            .iter()
            .position(|warrior| &warrior.metadata.name == name)
        {
            #[allow(
                clippy::indexing_slicing,
                reason = "`index` is guaranteed to be valid."
            )]
            Some(index) => {
                self.warriors[index] = warrior.clone();
            }
            None => {
                self.warriors.push(warrior.clone());
            }
        }

        self.sort_warriors_by_name();
    }

    fn sort_warriors_by_name(&mut self) {
        self.warriors
            .sort_by(|a, b| a.metadata.name.cmp(&b.metadata.name));
    }
}
