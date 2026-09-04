use egui_macroquad::egui;

use crate::game_context::renderer::{
    number_str_cache::NumberStrCache, texture_manager::TextureManager,
};

pub mod color;

mod number_str_cache;
mod texture_manager;

/// `Renderer` has 2 responsibilities related to rendering:
/// - providing texture resources.
/// - providing string representations of numbers.
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
    /// Get string representation of usize `num`.
    #[inline]
    pub fn usize_to_str(&self, num: usize) -> &str {
        self.number_str_cache.get_str(num)
    }

    /// Get string representation of i32 `num`.
    /// Note: The i32 is wrapped in range `[0, core_size - 1]`.
    #[allow(
        clippy::cast_sign_loss,
        clippy::as_conversions,
        reason = "This conversion is safe because `num` is always a small non-negative value."
    )]
    #[inline]
    pub fn i32_to_str(&self, num: i32) -> &str {
        self.number_str_cache.get_str(num as usize)
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
