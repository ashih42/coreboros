use std::fmt;

use crate::instruction::{modifier::Modifier, opcode::Opcode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Operation {
    pub opcode: Opcode,
    pub modifier: Modifier,
}

impl Operation {
    #[must_use]
    pub const fn new(opcode: Opcode, modifier: Modifier) -> Self {
        Self { opcode, modifier }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.opcode, self.modifier)

        // f.pad(&format!("{}.{}", self.opcode, self.modifier))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_to_string() {
        assert_eq!(
            Operation::new(Opcode::DAT, Modifier::AB).to_string(),
            "DAT.AB"
        );
    }
}
