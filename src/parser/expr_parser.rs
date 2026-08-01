use pest::pratt_parser::{Assoc::Left, Op, PrattParser};
use std::sync::LazyLock;

use crate::parser::redcode_parser::Rule::{
    self, add, divide, modulo, multiply, subtract, unary_minus,
};

/// `EXPR_PARSER` enforces the following standard arithmetic operator precedence:
/// - `add` and `subtract` have lowest precedence.
/// - `multiply`, `divide`, and `modulo` have higher precedence.
/// - `unary_minus` has the highest precedence.
pub static EXPR_PARSER: LazyLock<PrattParser<Rule>> = LazyLock::new(|| {
    PrattParser::new()
        .op(Op::infix(add, Left) | Op::infix(subtract, Left))
        .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left))
        .op(Op::prefix(unary_minus))
});
