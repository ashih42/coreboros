use anyhow::{Context as _, Result, bail};
use std::{fs, mem};

use crate::{
    instruction::Instruction,
    parser::{
        comment_scanner::{CommentKeyPattern, scan_comment},
        label_dictionary::LabelDictionary,
        redcode_line::RedcodeLine,
        redcode_parser,
    },
    warrior::{Warrior, warrior_metadata::WarriorMetadata},
};

pub struct WarriorBuilder {
    redcode: String,
    metadata: WarriorMetadata,
    lines: Vec<RedcodeLine>,
    label_dictionary: LabelDictionary,
}

impl WarriorBuilder {
    /// Try to construct a `Warrior` from input `redcode`.
    ///
    /// # Errors
    /// Will return `Err` if:
    /// - Input Redcode fails syntax or semantic analysis.
    /// - Warrior is not valid for any reason.
    pub fn from_text(redcode: &str) -> Result<Warrior> {
        Self::try_build(redcode).with_context(|| "Error in warrior".to_owned())
    }

    /// Try to construct a `Warrior` from input Redcode file at `path`.
    /// Note: This function may be deprecated and subject to removal when this app is fully migrated to wasm app.
    ///
    /// # Errors
    /// Will return `Err` if:
    /// - File cannot be opened or read.
    /// - Input Redcode fails syntax or semantic analysis.
    /// - Warrior is not valid for any reason.
    pub fn from_file(path: &str) -> Result<Warrior> {
        fs::read_to_string(path)
            .map_err(anyhow::Error::new) // Or your custom Error::from
            .and_then(|redcode| Self::try_build(&redcode))
            .with_context(|| format!("Error in warrior at: {path}"))
    }

    /// Try to construct a `Warrior`.
    ///
    /// # Errors
    /// Will return `Err` if:
    /// - Input Redcode fails syntax or semantic analysis.
    /// - Warrior is not valid for any reason.
    fn try_build(redcode: &str) -> Result<Warrior> {
        let mut lines = redcode_parser::parse_redcode(redcode)?;
        Self::truncate_after_end(&mut lines);

        // for line in &lines {
        //     log::info!("{line:#?}");
        // }

        let warrior_builder = Self {
            redcode: redcode.to_owned(),
            metadata: WarriorMetadata::default(),
            lines,
            label_dictionary: LabelDictionary::default(),
        };

        warrior_builder.build()
    }

    /// If a line with pseudo-instruction `END` is found, keep this line and remove all lines after it.
    #[inline]
    fn truncate_after_end(lines: &mut Vec<RedcodeLine>) {
        let end_index = lines.iter().position(|line| line.end_instruction.is_some());

        if let Some(index) = end_index {
            lines.truncate(index + 1);
        }
    }

    /// Consume `self` to try to build `Warrior`.
    ///
    /// # Errors
    /// Will return `Err` if:
    /// - Failure in first pass of semantic analysis.
    /// - Failure in second pass of semantic analysis.
    /// - `instructions` is empty.
    /// - `origin` is not valid.
    fn build(mut self) -> Result<Warrior> {
        // 1. First pass of semantic analysis.
        self.first_pass()?;

        // // 2. Second pass of semantic analysis.
        let (instructions, origin) = self.second_pass()?;

        // 3. Validate instructions is not empty.
        if instructions.is_empty() {
            bail!("No instructions in warrior");
        }

        // 4. Validate origin is valid.
        if !Self::validate_origin(origin, &instructions) {
            bail!(
                "Invalid program origin\n\
                Warrior contains {} instructions, but origin points to {origin}",
                instructions.len()
            );
        }

        #[allow(
            clippy::cast_sign_loss,
            reason = "`origin` has been validated to be non-negative."
        )]
        let warrior = Warrior::new(self.redcode, self.metadata, instructions, origin as usize);
        Ok(warrior)
    }

    /// Validate `origin` is in the valid range, and return `origin` as an unsigned type if it is valid.
    #[inline]
    fn validate_origin(origin: i32, instructions: &[Instruction]) -> bool {
        #[allow(
            clippy::cast_possible_truncation,
            clippy::cast_possible_wrap,
            reason = "`instructions` length will be capped."
        )]
        let instruction_size = instructions.len() as i32;

        (0..instruction_size).contains(&origin)
    }

    /// The first pass of semantic analysis involves:
    /// - Inserting label definitions in dictionary.
    /// - Updating warrior metadata from comments.
    fn first_pass(&mut self) -> Result<()> {
        let mut current_instruction_line_number = 0;

        for line in &self.lines {
            // Insert label definitions in dictionary.
            if line.label_definitions.is_some() {
                self.label_dictionary
                    .insert_label_definitions(line, current_instruction_line_number)?;
            }

            // Update instruction line counter.
            if line.instruction.is_some() {
                current_instruction_line_number += 1;
            }

            // Update warrrior metadata from comments.
            if let Some(comment) = &line.comment {
                Self::process_comment(&mut self.metadata, comment);
            }
        }

        Ok(())
    }

    /// Update `warrior_metadata` if `comment` defines `name`, `author`, or `strategy`.
    fn process_comment(warrior_metadata: &mut WarriorMetadata, comment: &str) {
        // Overwrite `name`.
        if let Some(name) = scan_comment(CommentKeyPattern::Name, comment) {
            name.clone_into(&mut warrior_metadata.name);
        }

        // Overwrite `author`.
        if let Some(author) = scan_comment(CommentKeyPattern::Author, comment) {
            warrior_metadata.author = Some(author.to_owned());
        }

        // Append to `strategy`.
        if let Some(strategy) = scan_comment(CommentKeyPattern::Strategy, comment) {
            let full_strategy = warrior_metadata.strategy.get_or_insert_with(String::new);

            if !full_strategy.is_empty() {
                full_strategy.push(' ');
            }
            full_strategy.push_str(strategy);
        }
    }

    /// The second pass of semantic analysis consumes `self.lines` to:
    /// - Resolving expressions to build concrete instructions.
    /// - Determining `origin` from pseudo-instructions.
    ///
    /// Return concrete instructions and a temporary signed `origin` to be validated.
    fn second_pass(&mut self) -> Result<(Vec<Instruction>, i32)> {
        let mut current_instruction_line_number = 0;

        let mut instructions = Vec::new();
        let mut origin = 0;

        for line in mem::take(&mut self.lines) {
            let line_number = line.text_line_number;

            Self::process_line_in_second_pass(
                line,
                &mut current_instruction_line_number,
                &mut instructions,
                &mut origin,
                &self.label_dictionary,
            )
            .with_context(|| format!("Error on line {line_number}:"))?;
        }

        Ok((instructions, origin))
    }

    fn process_line_in_second_pass(
        line: RedcodeLine,
        current_instruction_line_number: &mut usize,
        instructions: &mut Vec<Instruction>,
        origin: &mut i32,
        label_dictionary: &LabelDictionary,
    ) -> Result<()> {
        // Build the concrete `instruction`.
        if let Some(instruction_builder) = line.instruction {
            let instruction =
                instruction_builder.build(label_dictionary, *current_instruction_line_number)?;
            // .with_context(|| "Error on line {line.text_line_number}")?;

            instructions.push(instruction);
            *current_instruction_line_number += 1;
        }

        // Update `origin` from `ORG` instruction.
        if let Some(org) = line.org_instruction {
            *origin = org.eval_origin(label_dictionary)?;
        }

        // Update `origin` from `END` instruction if applicable.
        if let Some(end) = line.end_instruction
            && let Some(result) = end.eval_origin(label_dictionary)
        {
            *origin = result?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_file() {
        assert!(WarriorBuilder::from_file("warriors/no_such_file.red").is_err());
        assert!(WarriorBuilder::from_file("warriors/no_permission.red").is_err());
    }
}
