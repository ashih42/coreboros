use crate::{
    instruction::Instruction,
    parser::{warrior_builder::WarriorBuilder, warrior_error::WarriorError},
    warrior_metadata::WarriorMetadata,
};

pub type WarriorId = u8;

pub struct Warrior {
    // pub warrior_id: WarriorId, // maybe don't store it here, but store it in the hashmap mapping itself.
    pub metadata: WarriorMetadata,
    pub instructions: Vec<Instruction>,
    pub origin: usize,
}

impl Warrior {
    #[must_use]
    pub const fn new(
        metadata: WarriorMetadata,
        instructions: Vec<Instruction>,
        origin: usize,
    ) -> Self {
        Self {
            metadata,
            instructions,
            origin,
        }
    }

    ///
    /// # Errors
    /// Will return `Err` if:
    /// - Failure to construct `WarriorBuilder`.
    /// - Failure in `WarriorBuilder::.build()`.
    pub fn from_file(filepath: &str) -> Result<Self, WarriorError> {
        WarriorBuilder::new(filepath)?.build()
    }

    #[must_use]
    pub fn to_load_file(&self) -> String {
        format!(
            ";redcode\n\
            ;name          {name}\n\
            ;author        {author}\n\
            ;strategy      {strategy}\n\
            \n\
            org            {origin}\n\
            \n\
            {instructions}\n",
            name = self.metadata.name,
            author = self.metadata.author.as_deref().unwrap_or_default(),
            strategy = self.metadata.strategy.as_deref().unwrap_or_default(),
            origin = self.origin,
            instructions = self
                .instructions
                .iter()
                .map(Instruction::to_load_file)
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Default for Warrior {
    // for testing
    fn default() -> Self {
        let metadata = WarriorMetadata {
            name: "Joe".to_owned(),
            author: None,
            // version: None,
            // date: None,
            strategy: None,
        };
        Self {
            // warrior_id: 0,
            metadata,
            instructions: Vec::new(),
            origin: 0,
        }
    }
}
