use anyhow::{Context as _, Result, bail};
use pest::iterators::Pairs;

use crate::parser::{
    expr_parser::EXPR_PARSER, label_dictionary::LabelDictionary, redcode_parser::Rule,
};

/// `Expr` is a node in the expression tree, to be constructed in the first pass, and
/// to be evaluated in the second pass of semantic analysis.
#[derive(Debug)]
pub enum Expr {
    Integer(String),
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
        clippy::unreachable,
        reason = "The grammar guarantees only these rules may occur here."
    )]
    pub fn parse_expr(expr_pairs: Pairs<Rule>) -> Self {
        EXPR_PARSER
            .map_primary(|primary| match primary.as_rule() {
                Rule::integer => Self::Integer(primary.as_str().to_owned()),
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
    pub fn eval(&self, label_dictionary: &LabelDictionary, current_line: usize) -> Result<i32> {
        match self {
            Self::Integer(number_string) => number_string.parse::<i32>().map_err(Into::into),

            Self::Label(label) => Self::find_offset(label, label_dictionary, current_line),

            Self::UnaryMinus(expr) => {
                let value = expr.eval(label_dictionary, current_line)?;

                Ok((-1_i32).wrapping_mul(value))
            }

            Self::BinaryOperation { lhs, operator, rhs } => {
                let lhs = lhs.eval(label_dictionary, current_line)?;
                let rhs = rhs.eval(label_dictionary, current_line)?;

                match operator {
                    BinaryOperator::Add => Ok(lhs.wrapping_add(rhs)),
                    BinaryOperator::Subtract => Ok(lhs.wrapping_sub(rhs)),
                    BinaryOperator::Multiply => Ok(lhs.wrapping_mul(rhs)),
                    BinaryOperator::Divide => {
                        if rhs == 0 {
                            bail!("Division by zero");
                        }
                        #[allow(
                            clippy::arithmetic_side_effects,
                            reason = "This is safe because I checked for division by zero."
                        )]
                        Ok(lhs.wrapping_div(rhs))
                    }
                    BinaryOperator::Modulo => {
                        if rhs == 0 {
                            bail!("Modulo by zero");
                        }
                        #[allow(
                            clippy::arithmetic_side_effects,
                            reason = "This is safe because I checked for modulo by zero."
                        )]
                        Ok(lhs.wrapping_rem(rhs))
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
    pub fn eval_absolute_address(&self, label_dictionary: &LabelDictionary) -> Result<i32> {
        self.eval(label_dictionary, 0)
    }

    /// Return the number of lines to go from `current_line` to `label` line.
    #[inline]
    fn find_offset(
        label: &str,
        label_dictionary: &LabelDictionary,
        current_line: usize,
    ) -> Result<i32> {
        label_dictionary
            .get_relative_line_number(label, current_line)
            .with_context(|| format!("Undefined label: \"{label}\""))
    }
}

#[allow(clippy::unwrap_used)]
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

        assert!(matches!(answer, Ok(7)));
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

        assert!(answer.is_err());
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

        assert!(answer.is_err());
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

        assert!(answer.is_err());
    }
}
