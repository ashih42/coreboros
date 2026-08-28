use anyhow::Result;

use crate::{
    instruction::Instruction, parser::warrior_builder::WarriorBuilder,
    warrior::warrior_metadata::WarriorMetadata,
};

pub mod warrior_id;
pub mod warrior_metadata;

mod examples;

/// `Warrior` represents a valid compiled redcode program, ready for gameplay.
///
/// Note: The compiled `instructions` contain A and B numbers in a raw format, unaffected by core size.
/// `Mars` will wrap these numbers when they are loaded into the core.
#[derive(Clone)]
pub struct Warrior {
    pub redcode: String,
    pub metadata: WarriorMetadata,
    pub instructions: Vec<Instruction>,
    pub origin: usize,
}

impl Warrior {
    #[must_use]
    pub const fn new(
        redcode: String,
        metadata: WarriorMetadata,
        instructions: Vec<Instruction>,
        origin: usize,
    ) -> Self {
        Self {
            redcode,
            metadata,
            instructions,
            origin,
        }
    }

    /// Try to construct a `Warrior` from input text.
    ///
    /// # Errors
    /// Will return `Err` if Warrior is cannot be constructed.
    pub fn from_text(text: &str) -> Result<Self> {
        WarriorBuilder::from_text(text)
    }

    /// Try to construct a `Warrior` from input Redcode file at `path`.
    ///
    /// # Errors
    /// Will return `Err` if Warrior is cannot be constructed.
    pub fn from_file(path: &str) -> Result<Self> {
        WarriorBuilder::from_file(path)
    }

    /// Convert the warrior to Load File format.
    /// Reference: <https://corewar.co.uk/standards/icws94.htm#3.0>
    #[must_use]
    pub fn as_load_file(&self) -> String {
        indoc::formatdoc! {"
            ;redcode
            ;name          {name}
            ;author        {author}
            ;strategy      {strategy}
            
            org            {origin}
            
            {instructions}
            ",
            name = self.metadata.name,
            author = self.metadata.author.as_deref().unwrap_or_default(),
            strategy = self.metadata.strategy.as_deref().unwrap_or_default(),
            origin = self.origin,
            instructions = self
                .instructions
                .iter()
                .map(Instruction::as_load_file)
                .collect::<Vec<String>>()
                .join("\n")
        }
    }
}
