use pest::iterators::Pairs;

use crate::parser::{
    expr_error::ExprError, expr_parser::EXPR_PARSER, label_dictionary::LabelDictionary,
    redcode_parser::Rule,
};

/// `Expr` is a node in the expression tree, to be constructed in the first pass, and
/// to be evaluated in the second pass of semantic analysis.
#[derive(Debug)]
pub enum Expr {
    Integer(i32),
    Label(String),
    UnaryMinus(Box<Self>),
    BinaryOperation {
        lhs: Box<Self>,
        operator: BinaryOperator,
        rhs: Box<Self>,
    },
}

/// `BinaryOperator` includes all binary arithmetic operations that may occur inside `Expr`.
#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

impl Expr {
    /// Convert `expr_pairs` created from `RedcodeParser::parse(Rule::expr, input)`
    /// into a `Expr` tree with a nested hierarchy that encodes standard arithmetic operator precedence.
    #[allow(
        clippy::missing_panics_doc,
        reason = "These operations are guaranteed to succeed by the grammar."
    )]
    pub fn parse_expr(expr_pairs: Pairs<Rule>) -> Self {
        EXPR_PARSER
            .map_primary(|primary| match primary.as_rule() {
                Rule::integer => Self::Integer(primary.as_str().parse::<i32>().unwrap()),
                Rule::label => Self::Label(primary.as_str().to_owned()),
                Rule::expr => Self::parse_expr(primary.into_inner()),
                _ => unreachable!(),
            })
            .map_infix(|lhs, op, rhs| {
                let operator = match op.as_rule() {
                    Rule::add => BinaryOperator::Add,
                    Rule::subtract => BinaryOperator::Subtract,
                    Rule::multiply => BinaryOperator::Multiply,
                    Rule::divide => BinaryOperator::Divide,
                    Rule::modulo => BinaryOperator::Modulo,
                    _ => unreachable!(),
                };
                Self::BinaryOperation {
                    lhs: Box::new(lhs),
                    operator,
                    rhs: Box::new(rhs),
                }
            })
            .map_prefix(|op, rhs| match op.as_rule() {
                Rule::unary_minus => Self::UnaryMinus(Box::new(rhs)),
                _ => unreachable!(),
            })
            .parse(expr_pairs)
    }

    /// Evaluate the tree recursively, resolving any labels as relative addresses to `current_line`.
    ///
    /// # Errors
    /// Will return `Err` if:
    /// - a label is undefined.
    /// - attempt to perform division by zero.
    /// - attempt to perform modulo by zero.
    pub fn eval(
        &self,
        label_dictionary: &LabelDictionary,
        current_line: usize,
    ) -> Result<i32, ExprError> {
        match self {
            Self::Integer(i) => Ok(*i),

            Self::Label(label) => Self::find_offset(label, label_dictionary, current_line),

            Self::UnaryMinus(expr) => Ok(-expr.eval(label_dictionary, current_line)?),

            Self::BinaryOperation { lhs, operator, rhs } => {
                let lhs = lhs.eval(label_dictionary, current_line)?;
                let rhs = rhs.eval(label_dictionary, current_line)?;

                match operator {
                    BinaryOperator::Add => Ok(lhs + rhs),
                    BinaryOperator::Subtract => Ok(lhs - rhs),
                    BinaryOperator::Multiply => Ok(lhs * rhs),
                    BinaryOperator::Divide => {
                        if rhs == 0 {
                            Err(ExprError::DivisionByZero)
                        } else {
                            Ok(lhs / rhs)
                        }
                    }
                    BinaryOperator::Modulo => {
                        if rhs == 0 {
                            Err(ExprError::ModuloByZero)
                        } else {
                            Ok(lhs % rhs)
                        }
                    }
                }
            }
        }
    }

    /// Evaluate the tree recursively, resolving any labels as absolute addresses.
    ///
    /// # Errors
    /// Will return `Err` if:
    /// - a label is undefined.
    /// - attempt to perform division by zero.
    /// - attempt to perform modulo by zero.
    pub fn eval_absolute_address(
        &self,
        label_dictionary: &LabelDictionary,
    ) -> Result<i32, ExprError> {
        self.eval(label_dictionary, 0)
    }

    /// Return the number of lines to go from `current_line` to `label` line.
    #[inline]
    fn find_offset(
        label: &str,
        label_dictionary: &LabelDictionary,
        current_line: usize,
    ) -> Result<i32, ExprError> {
        label_dictionary
            .get_relative_line_number(label, current_line)
            .map_or_else(
                || {
                    Err(ExprError::UndefinedLabel {
                        label: label.to_owned(),
                    })
                },
                Ok,
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::redcode_parser::RedcodeParser;
    use pest::Parser as _;

    #[test]
    fn test_eval_integers() {
        let input = "1 + 2 * 3";
        let label_dictionary = LabelDictionary::default();
        let current_line = 0;

        let root = RedcodeParser::parse(Rule::expr, input)
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(root.as_rule(), Rule::expr);

        let expr = Expr::parse_expr(root.into_inner());
        let answer = expr.eval(&label_dictionary, current_line);

        assert_eq!(answer, Ok(7));
    }

    #[test]
    fn test_eval_division_by_zero() {
        let input = "1 + 2 / 0";
        let label_dictionary = LabelDictionary::default();
        let current_line = 0;

        let root = RedcodeParser::parse(Rule::expr, input)
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(root.as_rule(), Rule::expr);

        let expr = Expr::parse_expr(root.into_inner());
        let answer = expr.eval(&label_dictionary, current_line);

        assert_eq!(answer, Err(ExprError::DivisionByZero));
    }

    #[test]
    fn test_eval_modulo_by_zero() {
        let input = "1 + 2 % 0";
        let label_dictionary = LabelDictionary::default();
        let current_line = 0;

        let root = RedcodeParser::parse(Rule::expr, input)
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(root.as_rule(), Rule::expr);

        let expr = Expr::parse_expr(root.into_inner());
        let answer = expr.eval(&label_dictionary, current_line);

        assert_eq!(answer, Err(ExprError::ModuloByZero));
    }

    #[test]
    fn test_eval_undefined_label() {
        let undefined_label = "doge";
        let input = format!("1 + 2 * {undefined_label}");
        let label_dictionary = LabelDictionary::default();
        let current_line = 0;

        let root = RedcodeParser::parse(Rule::expr, &input)
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(root.as_rule(), Rule::expr);

        let expr = Expr::parse_expr(root.into_inner());
        let answer = expr.eval(&label_dictionary, current_line);

        assert_eq!(
            answer,
            Err(ExprError::UndefinedLabel {
                label: undefined_label.to_owned()
            })
        );
    }
}
