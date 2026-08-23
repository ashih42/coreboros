use egui_macroquad::egui;
use std::sync::{Arc, LazyLock};

use crate::scene::editor::syntax_highlighter::SyntaxHighlighter;

pub struct TextEditor {
    pub input_text: String,
    pub cached_input_text: String,
    pub cached_input_number_of_lines: usize,
    pub line_numbers_col: String,
    pub cached_galley: Option<Arc<egui::Galley>>,
}

const MIN_LINES_TO_SHOW: usize = 50;

impl Default for TextEditor {
    fn default() -> Self {
        Self {
            input_text: String::new(),
            cached_input_text: String::new(),
            cached_input_number_of_lines: 0,
            line_numbers_col: Self::generate_line_numbers_col(MIN_LINES_TO_SHOW),
            cached_galley: None,
        }
    }
}

impl TextEditor {
    /// Rebuild `line_numbers_col` if `input_text` changed such that it would affect the number of lines to show.
    pub fn update_line_numbers_col_if_changed(&mut self) {
        let number_of_lines = self.input_text.split('\n').count().max(MIN_LINES_TO_SHOW);

        if self.cached_input_number_of_lines != number_of_lines {
            self.cached_input_number_of_lines = number_of_lines;
            self.line_numbers_col = Self::generate_line_numbers_col(number_of_lines);
        }
    }

    /// Create the line numbers column string, with each line containing a number left-padded with space.
    fn generate_line_numbers_col(number_of_lines: usize) -> String {
        (1..=number_of_lines).map(|i| format!("{i:4}\n")).collect()
    }

    /// If `input` changed, invalidate the cache and construct a new galley.
    /// Note: This operation cannot be a method because the TextEditor's `input_text` is also borrowed mutably in another place.
    pub fn get_cached_or_build_new_galley(
        ui: &egui::Ui,
        input: &str,
        cached_input_text: &mut String,
        cached_galley: &mut Option<Arc<egui::Galley>>,
    ) -> Arc<egui::Galley> {
        if input != cached_input_text {
            *cached_input_text = input.to_owned();
            *cached_galley = None;
        }

        // Return cache or build new cached value.
        let galley = cached_galley.get_or_insert_with(|| Self::build_galley(ui, input));

        Arc::clone(galley)
    }

    /// Delegate `SyntaxHighlighter` to construct the `LayoutJob` and pass it to `ui` to construct a `Galley`.
    fn build_galley(ui: &egui::Ui, code: &str) -> Arc<egui::Galley> {
        static SYNTAX_HIGHLIGHTER: LazyLock<SyntaxHighlighter> =
            LazyLock::new(SyntaxHighlighter::default);

        let layout_job = SYNTAX_HIGHLIGHTER.highlight_text(code);

        ui.fonts(|f| f.layout_job(layout_job))
    }
}
