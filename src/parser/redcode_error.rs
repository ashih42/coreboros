use thiserror::Error;

use crate::parser::{expr_error::ExprError, redcode_parser::Rule};

#[derive(Error, Debug)]
pub enum RedcodeError {
    // Note: `pest_consume::Error<Rule>` is very big, so boxing it significantly reduces size of `RedcodeError`.
    #[error("Invalid syntax: {err}")]
    SyntaxError { err: Box<pest_consume::Error<Rule>> },

    #[error(
        "Duplicate label definition for \"{label}\"\n\
        First defined on line {first_defined_text_line_number}\n\
        Later redefined on line {later_redefined_text_line_number}"
    )]
    DuplicateLabelDefinition {
        label: String,
        first_defined_text_line_number: usize,
        later_redefined_text_line_number: usize,
    },

    #[error("Could not evaluate expression on line {line_number}: {err}")]
    ExprEvaluation { line_number: usize, err: ExprError },
}
