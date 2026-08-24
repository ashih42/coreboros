use crate::warrior::Warrior;

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
            Warrior::transposition_stone(), // TODO: Something might be wrong with my execution logic. Investigate!
            Warrior::self_bombing_stone(),
            Warrior::self_vamping_vampire(), // TODO: Something might be wrong with my execution logic. Investigate!
        ];

        let mut vault = Self { warriors };
        vault.sort_warriors_by_name();

        vault
    }
}

impl WarriorVault {
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

    pub fn save_warrior(&mut self, warrior: &Warrior) {
        let name = &warrior.metadata.name;

        match self
            .warriors
            .iter()
            .position(|warrior| &warrior.metadata.name == name)
        {
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
