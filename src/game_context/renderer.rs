use crate::game_context::renderer::{
    number_str_cache::NumberStrCache, texture_manager::TextureManager,
};

pub mod color;

mod number_str_cache;
mod style;
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
        let mut texture_manager = None;

        egui_macroquad::cfg(|egui_ctx| {
            style::apply_monospace_font_style(egui_ctx);
            texture_manager = Some(TextureManager::new(egui_ctx));
        });

        Self {
            #[allow(clippy::unwrap_used, reason = "This unwrap cannot fail.")]
            texture_manager: texture_manager.unwrap(),
            number_str_cache: NumberStrCache::default(),
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
}
