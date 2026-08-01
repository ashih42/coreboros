use thiserror::Error;

/// `ExprError` includes all possible situations where an `Expr` can fail to evaluate.
#[derive(Error, Debug, Eq, PartialEq)]
pub enum ExprError {
    #[error("Undefined label: \"{label}\"")]
    UndefinedLabel { label: String },

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Modulo by zero")]
    ModuloByZero,
}
