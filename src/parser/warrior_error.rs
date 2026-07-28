use std::io;
use std::path::PathBuf;
use thiserror::Error;

use crate::parser::redcode_error::RedcodeError;

#[derive(Error, Debug)]
pub enum WarriorError {
    #[error(
        "Could not open \"{filepath}\"\n\
        {err}"
    )]
    FileError { filepath: PathBuf, err: io::Error },

    #[error(
        "Redcode error in \"{filepath}\"\n\
        {err}"
    )]
    RedcodeError {
        filepath: PathBuf,
        err: RedcodeError,
    },

    #[error("No instructions in \"{filepath}\"")]
    EmptyInstructions { filepath: PathBuf },

    #[error(
        "Invalid program origin in \"{filepath}\"\n\
        There are {num_instructions} instructions, but origin is at {origin}"
    )]
    InvalidOrigin {
        filepath: PathBuf,
        num_instructions: usize,
        origin: i32,
    },
}
