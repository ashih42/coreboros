#[allow(unused_macros, reason = "This macro is only used for testing.")]
/// Check if the A or B operand at `cell_index` holds the `expected` value.
macro_rules! assert_operand_eq {
    ($mars:expr, $cell_index:literal, $operand:ident, $expected:expr) => {{
        assert_eq!(
            $mars
                .core
                .get_cell(CoreNumber::from_usize_unchecked($cell_index))
                .instruction
                .$operand
                .number
                .as_index(),
            $expected
        );
    }};
}

#[allow(unused_imports, reason = "This macro export is only used for testing.")]
pub(crate) use assert_operand_eq;
