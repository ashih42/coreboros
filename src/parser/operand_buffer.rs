use crate::{
    instruction::{addressing_mode::AddressingMode, operand::Operand},
    parser::{expr::Expr, expr_error::ExprError, label_dictionary::LabelDictionary},
};

#[derive(Debug)]
pub struct OperandBuffer {
    pub mode: Option<AddressingMode>,
    pub expr: Expr,
}

impl OperandBuffer {
    #[must_use]
    pub const fn new(mode: Option<AddressingMode>, expr: Expr) -> Self {
        Self { mode, expr }
    }

    /// Consume `self` to build an `Operand`.
    ///
    /// # Errors
    /// Will return `Err` if `expr` fails to evaluate.
    pub fn build(
        self,
        label_dictionary: &LabelDictionary,
        current_line: usize,
    ) -> Result<Operand, ExprError> {
        let mode = self.mode.unwrap_or_default();
        let number = self.expr.eval(label_dictionary, current_line)?;

        Ok(Operand::new(mode, number))
    }
}
