use egui_macroquad::egui;

use crate::warrior::warrior_id::WarriorId;

/// Get `macroquad::color::Color`, which is used to draw macroquad shapes to the game area.
#[inline]
pub const fn get_mq_color(maybe_warrior_id: Option<WarriorId>) -> macroquad::color::Color {
    let egui_color32 = get_egui_color32(maybe_warrior_id);

    macroquad::color::Color::from_rgba(
        egui_color32.r(),
        egui_color32.g(),
        egui_color32.b(),
        egui_color32.a(),
    )
}

/// Get `egui::Color32`, which is used to draw UI elements.
#[inline]
pub const fn get_egui_color32(maybe_warrior_id: Option<WarriorId>) -> egui::Color32 {
    const DARK_GREEN: egui::Color32 = egui::Color32::DARK_GREEN;
    const DARK_RED: egui::Color32 = egui::Color32::DARK_RED;
    const BLUE: egui::Color32 = egui::Color32::BLUE;
    const PURPLE: egui::Color32 = egui::Color32::PURPLE;
    const LIME_GREEN: egui::Color32 = egui::Color32::from_rgb(82, 138, 30);
    const ORANGE: egui::Color32 = egui::Color32::from_rgb(190, 74, 8);
    const PINK: egui::Color32 = egui::Color32::from_rgb(160, 60, 152);
    const BROWN: egui::Color32 = egui::Color32::from_rgb(98, 58, 24);
    const DARK_GRAY: egui::Color32 = egui::Color32::DARK_GRAY;

    match maybe_warrior_id {
        None => DARK_GRAY,
        Some(warrior_id) => match warrior_id.as_index() {
            0 => DARK_GREEN,
            1 => DARK_RED,
            2 => BLUE,
            3 => PURPLE,
            4 => LIME_GREEN,
            5 => ORANGE,
            6 => PINK,
            7 => BROWN,

            #[allow(
                clippy::unreachable,
                reason = "The engine guarantees at most 8 warriors."
            )]
            _ => unreachable!(),
        },
    }
}
