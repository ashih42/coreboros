/// `LabelDefinition` keep tracks of:
/// - `text_line_number` - the line number in the redcode file where a label was first defined (for reporting error).
/// - `instruction_line_number` - the line number of an instruction where a label points to (for resolving relative address).
pub struct LabelDefinition {
    pub text_line_number: usize,
    pub instruction_line_number: usize,
}

impl LabelDefinition {
    #[must_use]
    pub const fn new(text_line_number: usize, instruction_line_number: usize) -> Self {
        Self {
            text_line_number,
            instruction_line_number,
        }
    }
}
