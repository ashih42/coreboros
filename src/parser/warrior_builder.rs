use std::{fs, mem, path::PathBuf};

use crate::{
    instruction::Instruction,
    parser::{
        comment_scanner::{CommentKeyPattern, scan_comment},
        label_dictionary::LabelDictionary,
        redcode_error::RedcodeError,
        redcode_line::RedcodeLine,
        redcode_parser,
        warrior_error::WarriorError,
    },
    warrior::Warrior,
    warrior_metadata::WarriorMetadata,
};

pub struct WarriorBuilder {
    filepath: PathBuf,
    metadata: WarriorMetadata,
    lines: Vec<RedcodeLine>,
    label_dictionary: LabelDictionary,
}

impl WarriorBuilder {
    /// Try to construct a `WarriorBuilder` with partially populated data from input file.
    ///
    /// # Errors
    /// Will return `Err` if:
    /// - Fail to open/read file.
    /// - Input Redcode fails syntax analysis.
    pub fn new(filepath: &str) -> Result<Self, WarriorError> {
        let redcode = fs::read_to_string(filepath).map_err(|err| WarriorError::FileError {
            filepath: filepath.into(),
            err,
        })?;

        log::info!("Read input redcode from \"{filepath}\":\n\n{redcode}\n\n");

        let mut lines =
            redcode_parser::parse_redcode(&redcode).map_err(|err| WarriorError::RedcodeError {
                filepath: filepath.into(),
                err,
            })?;
        Self::truncate_after_end(&mut lines);

        // for line in &lines {
        //     log::info!("{line:#?}");
        // }

        Ok(Self {
            filepath: filepath.into(),
            metadata: WarriorMetadata::from_file(filepath),
            lines,
            label_dictionary: LabelDictionary::default(),
        })
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
    pub fn build(mut self) -> Result<Warrior, WarriorError> {
        // 1. First pass of semantic analysis.
        self.first_pass()
            .map_err(|err| WarriorError::RedcodeError {
                filepath: self.filepath.clone(),
                err,
            })?;

        // // 2. Second pass of semantic analysis.
        let (instructions, origin) =
            self.second_pass()
                .map_err(|err| WarriorError::RedcodeError {
                    filepath: self.filepath.clone(),
                    err,
                })?;

        // 3. Validate instructions is not empty.
        if instructions.is_empty() {
            return Err(WarriorError::EmptyInstructions {
                filepath: self.filepath.clone(),
            });
        }

        // 4. Validate origin is valid.
        if !Self::validate_origin(origin, &instructions) {
            return Err(WarriorError::InvalidOrigin {
                filepath: self.filepath.clone(),
                num_instructions: instructions.len(),
                origin,
            });
        }

        #[allow(
            clippy::cast_sign_loss,
            reason = "`origin` has been validated to be non-negative."
        )]
        let warrior = Warrior::new(self.metadata, instructions, origin as usize);
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
    fn first_pass(&mut self) -> Result<(), RedcodeError> {
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
    fn second_pass(&mut self) -> Result<(Vec<Instruction>, i32), RedcodeError> {
        let mut current_instruction_line_number = 0;

        let mut instructions = Vec::new();
        let mut origin = 0;

        for line in mem::take(&mut self.lines) {
            // Build the concrete `instruction`.
            if let Some(instruction_builder) = line.instruction {
                let instruction = instruction_builder
                    .build(&self.label_dictionary, current_instruction_line_number)
                    .map_err(|err| RedcodeError::ExprEvaluation {
                        line_number: line.text_line_number,
                        err,
                    })?;

                instructions.push(instruction);
                current_instruction_line_number += 1;
            }

            // Update `origin` from `ORG` instruction.
            if let Some(org) = line.org_instruction {
                origin = org.eval_origin(&self.label_dictionary).map_err(|err| {
                    RedcodeError::ExprEvaluation {
                        line_number: line.text_line_number,
                        err,
                    }
                })?;
            }

            // Update `origin` from `END` instruction if applicable.
            if let Some(end) = line.end_instruction
                && let Some(result) = end.eval_origin(&self.label_dictionary)
            {
                origin = result.map_err(|err| RedcodeError::ExprEvaluation {
                    line_number: line.text_line_number,
                    err,
                })?;
            }
        }

        Ok((instructions, origin))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_file() {
        assert!(matches!(
            WarriorBuilder::new("warriors/no_such_file.red"),
            Err(WarriorError::FileError { .. })
        ));

        assert!(matches!(
            WarriorBuilder::new("warriors/no_permission.red"),
            Err(WarriorError::FileError { .. })
        ));
    }
}
