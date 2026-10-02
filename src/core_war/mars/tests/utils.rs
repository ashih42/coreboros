/// Check if `mars` has a cell at `cell_index` with its A or B operand equal to the `expected` value.
#[macro_export]
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
