use egui_macroquad::egui;
use regex::{Regex, RegexBuilder};

use crate::scene::editor::syntax_kind::SyntaxKind;

pub struct SyntaxHighlighter {
    rules: Vec<(Regex, SyntaxKind)>,
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        #[allow(clippy::expect_used, reason = "Regex is valid 👌")]
        let operation_regex =
            RegexBuilder::new(r"\b(DAT|MOV|ADD|SUB|MUL|DIV|MOD|JMP|JMZ|JMN|DJN|SPL|CMP|SEQ|SNE|SLT|LDP|STP|NOP)(\.(A|B|AB|BA|F|X|I))?\b")
            .case_insensitive(true)
            .build()
            .expect("`operation_regex` regex pattern should be valid");

        #[allow(clippy::expect_used, reason = "Regex is valid 👌")]
        let pseudo_opcode_regex = RegexBuilder::new(r"\b(ORG|END)\b")
            .case_insensitive(true)
            .build()
            .expect("`pseudo_opcode_regex` regex pattern should be valid");

        #[allow(clippy::expect_used, reason = "Regex is valid 👌")]
        let addressing_mode_regex = Regex::new(r"[#$*@{}<>]")
            .expect("`addressing_mode_regex` regex pattern should be valid");

        #[allow(clippy::expect_used, reason = "Regex is valid 👌")]
        let comment_regex =
            Regex::new(r"(?m);.*$").expect("`comment_regex` regex pattern should be valid");

        let rules = vec![
            (operation_regex, SyntaxKind::Operation),
            (pseudo_opcode_regex, SyntaxKind::PseudoOpcode),
            (addressing_mode_regex, SyntaxKind::AddressingMode),
            (comment_regex, SyntaxKind::Comment),
        ];

        Self { rules }
    }
}

impl SyntaxHighlighter {
    #[allow(
        clippy::indexing_slicing,
        clippy::string_slice,
        reason = "Indices are valid 👌"
    )]
    pub fn highlight_text(&self, text: &str) -> egui::text::LayoutJob {
        let mut layout_job = egui::text::LayoutJob::default();

        // Assume every single character defaults to `Other`.
        let mut syntax_kinds = vec![SyntaxKind::Other; text.len()];

        // Overwrite each character's syntax_kind, for all characters in a regex match.
        for (regex, syntax_kind) in &self.rules {
            for mat in regex.find_iter(text) {
                for i in mat.start()..mat.end() {
                    if i < syntax_kinds.len() {
                        syntax_kinds[i] = *syntax_kind;
                    }
                }
            }
        }

        if !text.is_empty() {
            let mut start = 0;
            let mut current_syntax = syntax_kinds[0];

            // Merge blocks of the same adjacent syntax_kinds into TextFormat entries in `layout_job`.
            for (i, &syntax) in syntax_kinds.iter().enumerate() {
                if syntax != current_syntax {
                    layout_job.append(&text[start..i], 0.0, current_syntax.as_text_format());
                    start = i;
                    current_syntax = syntax;
                }
            }
            // Append the final remaining text block
            layout_job.append(&text[start..], 0.0, current_syntax.as_text_format());
        }

        layout_job.wrap.max_width = f32::INFINITY;
        layout_job
    }
}
