use egui_macroquad::egui;

use crate::renderer::{number_str_cache::NumberStrCache, texture_manager::TextureManager};

mod number_str_cache;
mod texture_manager;

pub struct Renderer {
    pub texture_manager: TextureManager,
    number_str_cache: NumberStrCache,
}

impl Default for Renderer {
    fn default() -> Self {
        let mut texture_manager = TextureManager::default();
        let number_str_cache = NumberStrCache::default();

        egui_macroquad::cfg(|egui_ctx| {
            Self::apply_monospace_font_style(egui_ctx);
            texture_manager.register_textures(egui_ctx);
        });

        Self {
            texture_manager,
            number_str_cache,
        }
    }
}

impl Renderer {
    #[inline]
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        clippy::as_conversions,
        reason = "This conversion is safe because `num` is small."
    )]
    pub fn usize_to_str(&self, num: usize) -> &str {
        self.number_str_cache.i32_to_str(num as i32)
    }

    #[inline]
    #[allow(
        clippy::arithmetic_side_effects,
        reason = "This expression is safe because `num` is small."
    )]
    pub fn usize_plus_1_to_str(&self, num: usize) -> &str {
        self.usize_to_str(num + 1)
    }

    #[inline]
    pub fn i32_to_str(&self, num: i32) -> &str {
        self.number_str_cache.i32_to_str(num)
    }

    /// Override the UI context style to use monospace fonts.
    fn apply_monospace_font_style(egui_ctx: &egui::Context) {
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
}
