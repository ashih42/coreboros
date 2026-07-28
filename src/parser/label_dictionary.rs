use std::collections::HashMap;

use crate::parser::{
    label_definition::LabelDefinition, redcode_error::RedcodeError, redcode_line::RedcodeLine,
};

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
    ) -> Result<(), RedcodeError> {
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
    ) -> Result<(), RedcodeError> {
        if let Some(existing_definition) = self.dictionary.get(label) {
            return Err(RedcodeError::DuplicateLabelDefinition {
                label: label.to_owned(),
                first_defined_text_line_number: existing_definition.text_line_number,
                later_redefined_text_line_number: text_line_number,
            });
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
            reason = "Number of Redcode instructions will be limited, so numbers can't get that high."
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
