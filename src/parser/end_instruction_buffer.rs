use anyhow::Result;

use crate::parser::{expr::Expr, label_dictionary::LabelDictionary};

#[derive(Debug)]
pub struct EndInstructionBuffer {
    pub expr: Option<Expr>,
}

impl EndInstructionBuffer {
    #[must_use]
    pub const fn new(expr: Option<Expr>) -> Self {
        Self { expr }
    }

    /// Consume `self` to evaluate and return the program origin if defined.
    ///
    /// # Errors
    /// Will return `Err` if `expr` fails to evaluate.
    #[must_use]
    pub fn eval_origin(self, label_dictionary: &LabelDictionary) -> Option<Result<i32>> {
        self.expr
            .as_ref()
            .map(|expr| expr.eval_absolute_address(label_dictionary))
    }
}
