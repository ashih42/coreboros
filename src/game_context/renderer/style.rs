use egui_macroquad::egui;

/// Override the UI context style to use monospace fonts.
pub fn apply_monospace_font_style(egui_ctx: &egui::Context) {
    let mut style = (*egui_ctx.style()).clone();

    style
        .text_styles
        .insert(egui::TextStyle::Heading, egui::FontId::monospace(22.0));

    style
        .text_styles
        .insert(egui::TextStyle::Body, egui::FontId::monospace(14.0));

    style
        .text_styles
        .insert(egui::TextStyle::Button, egui::FontId::monospace(14.0));

    style
        .text_styles
        .insert(egui::TextStyle::Small, egui::FontId::monospace(11.0));

    egui_ctx.set_style(style);
}
