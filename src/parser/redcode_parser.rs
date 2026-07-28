use pest_consume::Parser;
use std::str::FromStr as _;

use crate::{
    instruction::{addressing_mode::AddressingMode, modifier::Modifier, opcode::Opcode},
    parser::{
        end_instruction_buffer::EndInstructionBuffer, expr::Expr,
        instruction_builder::InstructionBuilder, operand_buffer::OperandBuffer,
        org_instruction_buffer::OrgInstructionBuffer, redcode_error::RedcodeError,
        redcode_line::RedcodeLine,
    },
};

#[derive(pest_derive::Parser)]
#[grammar = "grammar/redcode.pest"]
pub struct RedcodeParser;

type Node<'i> = pest_consume::Node<'i, Rule, ()>;

/// Implement handlers for each grammar production.
#[pest_consume::parser]
impl RedcodeParser {
    #[allow(
        clippy::used_underscore_binding,
        reason = "`pest_consume` requires this definition."
    )]
    fn EOI(_input: Node) {}

    /// At the top-level, convert a `file` nonterminal to `Vec<RedcodeLine>`.
    fn file(input: Node) -> Vec<RedcodeLine> {
        input
            .into_children()
            .filter(|child| child.as_rule() == Rule::line) // filter out the EOI node
            .map(|child| Self::line(child))
            .collect()
    }

    fn line(input: Node) -> RedcodeLine {
        let (text_line_number, _) = input.as_span().start_pos().line_col();

        let mut label_definitions = None;
        let mut instruction = None;
        let mut org_instruction = None;
        let mut end_instruction = None;
        let mut comment = None;

        for child in input.into_children() {
            match child.as_rule() {
                Rule::label_definitions => label_definitions = Some(Self::label_definitions(child)),
                Rule::instruction => instruction = Some(Self::instruction(child)),
                Rule::org_instruction => org_instruction = Some(Self::org_instruction(child)),
                Rule::end_instruction => end_instruction = Some(Self::end_instruction(child)),
                Rule::comment => comment = Some(Self::comment(child)),
                _ => unreachable!(),
            }
        }

        RedcodeLine {
            text_line_number,
            label_definitions,
            instruction,
            org_instruction,
            end_instruction,
            comment,
        }
    }

    fn org_instruction(input: Node) -> OrgInstructionBuffer {
        let mut children = input.into_children();

        let first_child = children.next().unwrap();
        let expr = Self::expr(first_child);

        OrgInstructionBuffer::new(expr)
    }

    fn end_instruction(input: Node) -> EndInstructionBuffer {
        let mut children = input.into_children();

        let first_child = children.next();
        let expr = first_child.map(Self::expr);

        EndInstructionBuffer::new(expr)
    }

    fn label_definitions(input: Node) -> Vec<String> {
        input.into_children().map(Self::label_definition).collect()
    }

    fn label_definition(input: Node) -> String {
        let child = input.into_children().single().unwrap();
        Self::label(child)
    }

    fn label(input: Node) -> String {
        input.as_str().to_owned()
    }

    fn comment(input: Node) -> String {
        input.as_str().to_owned()
    }

    fn instruction(input: Node) -> InstructionBuilder {
        let mut children = input.into_children();

        let first_child = children.next().unwrap();
        let (opcode, modifier) = Self::operation(first_child);

        let second_child = children.next().unwrap();
        let operand_1 = Self::operand(second_child);

        let operand_2 = children.next().map(Self::operand);

        InstructionBuilder {
            opcode,
            modifier,
            operand_1,
            operand_2,
        }
    }

    fn operation(input: Node) -> (Opcode, Option<Modifier>) {
        let mut children = input.into_children();

        let first_child = children.next().unwrap();
        let opcode = Self::opcode(first_child);

        let modifier = children
            .next()
            .map(|node_modifier| Self::modifier(node_modifier));

        (opcode, modifier)
    }

    fn opcode(input: Node) -> Opcode {
        Opcode::from_str(input.as_str()).unwrap()
    }

    fn modifier(input: Node) -> Modifier {
        Modifier::from_str(input.as_str()).unwrap()
    }

    fn operand(input: Node) -> OperandBuffer {
        let mut children = input.into_children();
        let first_node = children.next().unwrap();

        match first_node.as_rule() {
            Rule::mode => {
                let mode = Self::mode(first_node);
                let expr = Self::expr(children.next().unwrap());
                OperandBuffer::new(Some(mode), expr)
            }
            Rule::expr => {
                let expr = Self::expr(first_node);
                OperandBuffer::new(None, expr)
            }
            _ => unreachable!(),
        }
    }

    fn mode(input: Node) -> AddressingMode {
        AddressingMode::from_str(input.as_str()).unwrap()
    }

    fn expr(input: Node) -> Expr {
        let root = input.as_pair().clone();
        Expr::parse_expr(root.into_inner())
    }
}

/// Validate the syntax, parse `redcode` text into a tree structure, and convert the tree to `Vec<RedcodeLine`
/// for further semantic analysis.
///
/// # Errors
/// Will return `Err` if input `redcode` contains syntax errors caught by the grammar.
pub fn parse_redcode(redcode: &str) -> Result<Vec<RedcodeLine>, RedcodeError> {
    let nodes = RedcodeParser::parse(Rule::file, redcode)
        .map_err(|err| RedcodeError::SyntaxError { err: Box::new(err) })?;

    #[allow(
        clippy::missing_panics_doc,
        reason = "The tree structure is guaranteed by the grammar."
    )]
    let root = nodes.single().unwrap();
    let lines = RedcodeParser::file(root);

    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[allow(unused)]
    fn print_result(result: &Result<Vec<RedcodeLine>, RedcodeError>) {
        match &result {
            Ok(lines) => println!("lines:\n{:#?}", lines),
            Err(err) => println!("error:\n{}", err),
        }
    }

    #[test]
    fn test_parse_valid_instructions_and_label_definitions() {
        let redcode = indoc! {r#"
            DAT.F   #0, #0
            MOV.I   $1, >2
            ADD.AB  #5, @10
            SUB.BA  *1, #2
            MUL.A   {0, }1
            DIV.B   <0, <1
            MOD.X   $3, $4
            JMP.B   $5
            JMZ.F   3, 4+dog
            JMN.I   -5, -10
            DJN.A   -1, #0
            SPL.B   0
            SEQ.AB  #1, $2
            SNE.X   @0, *0
            SLT.BA  }1, {2
            NOP     0
            adder50
            adder
            add     er
        "#};

        let result = parse_redcode(redcode);
        // print_result(&result);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_valid_pseudoinstructions_and_label_definitions() {
        let redcode = indoc! {r#"
            ORGdoge
            org     dog+5
            END3    
            organ
            ender
            end     10
            end
        "#};

        let result = parse_redcode(redcode);
        // print_result(&result);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_all_combinations_of_label_definitions_instruction_comment() {
        let redcode = indoc! {r#"
                                             
                                    ;comment1
                        dat 2                
                        dat 3       ;comment3
            hello4                           
            hello5                  ;comment5
            hello6      dat 6                
            hello7      dat 7       ;comment7
        "#};

        let lines = parse_redcode(redcode).unwrap();
        assert_eq!(lines.len(), 8);

        assert!(matches!(
            lines[0],
            RedcodeLine {
                text_line_number: 1,
                label_definitions: None,
                instruction: None,
                comment: None,
                ..
            }
        ));

        assert!(matches!(
            lines[1],
            RedcodeLine {
                text_line_number: 2,
                label_definitions: None,
                instruction: None,
                comment: Some(..),
                ..
            }
        ));

        assert!(matches!(
            lines[2],
            RedcodeLine {
                text_line_number: 3,
                label_definitions: None,
                instruction: Some(..),
                comment: None,
                ..
            }
        ));

        assert!(matches!(
            lines[3],
            RedcodeLine {
                text_line_number: 4,
                label_definitions: None,
                instruction: Some(..),
                comment: Some(..),
                ..
            }
        ));

        assert!(matches!(
            lines[4],
            RedcodeLine {
                text_line_number: 5,
                label_definitions: Some(..),
                instruction: None,
                comment: None,
                ..
            }
        ));

        assert!(matches!(
            lines[5],
            RedcodeLine {
                text_line_number: 6,
                label_definitions: Some(..),
                instruction: None,
                comment: Some(..),
                ..
            }
        ));

        assert!(matches!(
            lines[6],
            RedcodeLine {
                text_line_number: 7,
                label_definitions: Some(..),
                instruction: Some(..),
                comment: None,
                ..
            }
        ));

        assert!(matches!(
            lines[7],
            RedcodeLine {
                text_line_number: 8,
                label_definitions: Some(..),
                instruction: Some(..),
                comment: Some(..),
                ..
            }
        ));
    }
}
