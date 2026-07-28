use crate::parser::{
    end_instruction_buffer::EndInstructionBuffer, instruction_builder::InstructionBuilder,
    org_instruction_buffer::OrgInstructionBuffer,
};

#[derive(Debug)]
pub struct RedcodeLine {
    pub text_line_number: usize,
    pub label_definitions: Option<Vec<String>>,
    pub instruction: Option<InstructionBuilder>,
    pub org_instruction: Option<OrgInstructionBuffer>,
    pub end_instruction: Option<EndInstructionBuffer>,
    pub comment: Option<String>,
}
