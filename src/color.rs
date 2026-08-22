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
    const DARK_GREEN: egui::Color32 = egui::Color32::DARK_GREEN;
    const DARK_RED: egui::Color32 = egui::Color32::DARK_RED;
    const BLUE: egui::Color32 = egui::Color32::BLUE;
    const PURPLE: egui::Color32 = egui::Color32::PURPLE;
    const LIME_GREEN: egui::Color32 = egui::Color32::from_rgb(82, 138, 30);
    const ORANGE: egui::Color32 = egui::Color32::from_rgb(190, 74, 8);
    const PINK: egui::Color32 = egui::Color32::from_rgb(160, 60, 152);
    const BROWN: egui::Color32 = egui::Color32::from_rgb(98, 58, 24);
    const DARK_GRAY: egui::Color32 = egui::Color32::DARK_GRAY;

    match warrior_id {
        Some(0) => DARK_GREEN,
        Some(1) => DARK_RED,
        Some(2) => BLUE,
        Some(3) => PURPLE,
        Some(4) => LIME_GREEN,
        Some(5) => ORANGE,
        Some(6) => PINK,
        Some(7) => BROWN,
        _ => DARK_GRAY,
    }
}
