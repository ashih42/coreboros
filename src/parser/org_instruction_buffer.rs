use crate::parser::{expr::Expr, expr_error::ExprError, label_dictionary::LabelDictionary};

#[derive(Debug)]
pub struct OrgInstructionBuffer {
    pub expr: Expr,
}

impl OrgInstructionBuffer {
    #[must_use]
    pub const fn new(expr: Expr) -> Self {
        Self { expr }
    }

    /// Consume `self` to evaluate and return the program origin.
    ///
    /// # Errors
    /// Will return `Err` if `expr` fails to evaluate.
    pub fn eval_origin(self, label_dictionary: &LabelDictionary) -> Result<i32, ExprError> {
        self.expr.eval_absolute_address(label_dictionary)
    }
}
