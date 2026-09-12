use egui_macroquad::egui;

/// `SyntaxKind` defines the syntactic entities and their visual representation, used by `SyntaxHighlighter`.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum SyntaxKind {
    AddressingMode,
    Comment,
    Operation,
    Other,
    PseudoOpcode,
}

impl SyntaxKind {
    /// Return the `egui::TextFormat` detailing how to format and color the text of this `SyntaxKind`.
    pub fn as_text_format(self) -> egui::TextFormat {
        let default_text_format = egui::TextFormat {
            font_id: egui::FontId::new(16.0, egui::FontFamily::Monospace),
            ..Default::default()
        };

        match self {
            Self::AddressingMode => egui::TextFormat {
                color: egui::Color32::from_rgb(255, 115, 0), // Neon Orange
                ..default_text_format
            },
            Self::Comment => egui::TextFormat {
                color: egui::Color32::DARK_GRAY,
                ..default_text_format
            },
            Self::Operation => egui::TextFormat {
                color: egui::Color32::from_rgb(0, 200, 255), // Cyber Cyan
                ..default_text_format
            },
            Self::Other => egui::TextFormat {
                color: egui::Color32::from_rgb(230, 230, 230), // Off White
                ..default_text_format
            },
            Self::PseudoOpcode => egui::TextFormat {
                color: egui::Color32::MAGENTA,
                ..default_text_format
            },
        }
    }
}
