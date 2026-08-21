use egui_macroquad::egui;

use crate::warrior::warrior_id::WarriorId;

/// Get `macroquad::color::Color`, which is used to draw macroquad shapes to the game area.
#[inline]
pub const fn get_mq_color(warrior_id: Option<WarriorId>) -> macroquad::color::Color {
    let egui_color32 = get_egui_color32(warrior_id);

    macroquad::color::Color::from_rgba(
        egui_color32.r(),
        egui_color32.g(),
        egui_color32.b(),
        egui_color32.a(),
    )
}

/// Get `egui::Color32`, which is used to draw UI elements.
#[inline]
pub const fn get_egui_color32(warrior_id: Option<WarriorId>) -> egui::Color32 {
    match warrior_id {
        Some(0) => egui::Color32::DARK_GREEN,
        Some(1) => egui::Color32::DARK_RED,
        Some(2) => egui::Color32::BLUE,
        Some(3) => egui::Color32::PURPLE,
        _ => egui::Color32::DARK_GRAY,
    }
}
