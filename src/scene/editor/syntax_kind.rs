use egui_macroquad::egui;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum SyntaxKind {
    AddressingMode,
    Comment,
    Operation,
    Other,
    PseudoOpcode,
}

impl SyntaxKind {
    pub fn as_text_format(&self) -> egui::TextFormat {
        match self {
            Self::AddressingMode => egui::TextFormat {
                font_id: egui::FontId::new(16.0, egui::FontFamily::Monospace),
                color: egui::Color32::from_rgb(255, 115, 0), // Neon Orange
                ..Default::default()
            },
            Self::Comment => egui::TextFormat {
                font_id: egui::FontId::new(16.0, egui::FontFamily::Monospace),
                color: egui::Color32::DARK_GRAY,
                ..Default::default()
            },
            Self::Operation => egui::TextFormat {
                font_id: egui::FontId::new(16.0, egui::FontFamily::Monospace),
                color: egui::Color32::from_rgb(0, 200, 255), // Cyber Cyan
                ..Default::default()
            },
            Self::Other => egui::TextFormat {
                font_id: egui::FontId::new(16.0, egui::FontFamily::Monospace),
                color: egui::Color32::from_rgb(230, 230, 230), // Off White
                ..Default::default()
            },
            Self::PseudoOpcode => egui::TextFormat {
                font_id: egui::FontId::new(16.0, egui::FontFamily::Monospace),
                color: egui::Color32::MAGENTA,
                ..Default::default()
            },
        }
    }
}
