use anyhow::{Result, bail};
use std::collections::HashMap;

use crate::parser::{label_definition::LabelDefinition, redcode_line::RedcodeLine};

#[derive(Default)]
pub struct LabelDictionary {
    dictionary: HashMap<String, LabelDefinition>,
}

impl LabelDictionary {
    /// Insert all labels defined in `line` into dictionary.
    ///
    /// # Errors
    /// Will return `Err` if any `label` defined in this `line` cannot be inserted.
    pub fn insert_label_definitions(
        &mut self,
        line: &RedcodeLine,
        current_line: usize,
    ) -> Result<()> {
        if let Some(labels) = &line.label_definitions {
            for label in labels {
                self.insert_label_definition(label, current_line, line.text_line_number)?;
            }
        }
        Ok(())
    }

    /// Insert the `label` into dictionary if an entry does not exist.
    /// If an entry exists, return an error.
    fn insert_label_definition(
        &mut self,
        label: &str,
        instruction_line_number: usize,
        text_line_number: usize,
    ) -> Result<()> {
        if let Some(existing_definition) = self.dictionary.get(label) {
            bail!(
                "Duplicate label definition for \"{label}\"\n\
                First defined on line {first_def}\n\
                Later redefined on line {second_def}",
                first_def = existing_definition.text_line_number,
                second_def = text_line_number
            );
        }

        self.dictionary.insert(
            label.to_owned(),
            LabelDefinition::new(text_line_number, instruction_line_number),
        );
        Ok(())
    }

    /// Return the number of lines to go from `current_line` to `label`.
    /// Return None if `label` is not defined.
    #[must_use]
    pub fn get_relative_line_number(&self, label: &str, current_line: usize) -> Option<i32> {
        #[allow(
            clippy::cast_possible_wrap,
            clippy::cast_possible_truncation,
            clippy::as_conversions,
            reason = "These conversions are safe 👌"
        )]
        #[allow(
            clippy::arithmetic_side_effects,
            reason = "The subtraction operation is safe 👌"
        )]
        self.get_line_number(label).map(|target| {
            let target = target as i32;
            let current = current_line as i32;
            target - current
        })
    }

    /// Return the `instruction_line_number` for `label`.
    /// Return None if `label` is not defined.
    fn get_line_number(&self, label: &str) -> Option<usize> {
        self.dictionary
            .get(label)
            .map(|definition| definition.instruction_line_number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_relative_line_number() {
        let label_dict = LabelDictionary {
            dictionary: HashMap::from([("cat".to_owned(), LabelDefinition::new(0, 10))]),
        };

        assert_eq!(label_dict.get_relative_line_number("cat", 0), Some(10));
        assert_eq!(label_dict.get_relative_line_number("cat", 10), Some(0));
        assert_eq!(label_dict.get_relative_line_number("cat", 20), Some(-10));
        assert_eq!(label_dict.get_relative_line_number("dog", 0), None);
    }

    #[test]
    fn test_get_line_number() {
        let label_dict = LabelDictionary {
            dictionary: HashMap::from([("cat".to_owned(), LabelDefinition::new(0, 10))]),
        };

        assert_eq!(label_dict.get_line_number("cat"), Some(10));
        assert_eq!(label_dict.get_line_number("dog"), None);
    }
}
