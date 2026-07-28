// use std::io;
// use thiserror::Error;

// #[derive(Error, Debug)]
// pub enum DataStoreError {
//     #[error("Reference to undefined label \"{label}\" on line {line_number}")]
//     UndefinedLabel { label: String, line_number: usize },

//     #[error("Division by zero on line {line_number}")]
//     DivisionByZero { line_number: usize },

//     #[error("Modulo by zero on line {line_number}")]
//     ModuloByZero { line_number: usize },

//     #[error("data store disconnected")]
//     Disconnect(#[from] io::Error),

//     #[error("the data for key `{0}` is not available")]
//     Redaction(String),

//     #[error("invalid header (expected {expected:?}, found {found:?})")]
//     InvalidHeader { expected: String, found: String },

//     #[error("unknown data store error")]
//     Unknown,
// }
