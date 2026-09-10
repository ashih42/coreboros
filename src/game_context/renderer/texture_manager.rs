use egui_macroquad::egui::{ColorImage, Context, TextureHandle};
use macroquad::{prelude::ImageFormat, texture::Image};

use crate::warrior::warrior_id::WarriorId;

/// `TextureManager` is responsible for initializing and providing texture resources.
pub struct TextureManager {
    pub trophy: TextureHandle,
    pub skull: TextureHandle,
    digit_1: TextureHandle,
    digit_2: TextureHandle,
    digit_3: TextureHandle,
    digit_4: TextureHandle,
    digit_5: TextureHandle,
    digit_6: TextureHandle,
    digit_7: TextureHandle,
    digit_8: TextureHandle,
}

/// Convert bytes to image, and register the image with the UI context.
/// Although bytes are available at compile time, registration with UI context must be done at runtime.
macro_rules! init_tex {
    ($egui_ctx:ident, $name:ident) => {{
        // Load image to bytes.
        let bytes = include_bytes!(concat!(
            "../../../assets/images/",
            stringify!($name),
            ".png"
        ));

        // Convert bytes to macroquad image.
        #[allow(clippy::panic, reason = "A malformed PNG file must trigger a panic.")]
        let mq_image =
            Image::from_file_with_format(bytes, Some(ImageFormat::Png)).unwrap_or_else(|_| {
                panic!("Failed to decode PNG asset: {}.png", stringify!($name));
            });

        // Convert macroquad image to egui's color image.
        #[allow(clippy::as_conversions, reason = "These conversions are safe 👌")]
        let color_image = ColorImage::from_rgba_unmultiplied(
            [mq_image.width as usize, mq_image.height as usize],
            &mq_image.bytes,
        );

        // Register this color image with the UI context.
        $egui_ctx.load_texture(stringify!($name), color_image, Default::default())
    }};
}

impl TextureManager {
    /// Initialize all textures and register them with UI context.
    pub fn new(egui_ctx: &Context) -> Self {
        Self {
            trophy: init_tex!(egui_ctx, trophy),
            skull: init_tex!(egui_ctx, skull),
            digit_1: init_tex!(egui_ctx, digit_1),
            digit_2: init_tex!(egui_ctx, digit_2),
            digit_3: init_tex!(egui_ctx, digit_3),
            digit_4: init_tex!(egui_ctx, digit_4),
            digit_5: init_tex!(egui_ctx, digit_5),
            digit_6: init_tex!(egui_ctx, digit_6),
            digit_7: init_tex!(egui_ctx, digit_7),
            digit_8: init_tex!(egui_ctx, digit_8),
        }
    }
}

impl TextureManager {
    /// Return the texture corresponding to `warrior_id`.
    /// Note: Because `warrior_id` is 0-based, and display is 1-based, it is necessary to
    /// return `digit_1` for warrior 0, and so on.
    pub const fn get_warrior_icon(&self, warrior_id: WarriorId) -> &TextureHandle {
        match warrior_id {
            WarriorId(0) => &self.digit_1,
            WarriorId(1) => &self.digit_2,
            WarriorId(2) => &self.digit_3,
            WarriorId(3) => &self.digit_4,
            WarriorId(4) => &self.digit_5,
            WarriorId(5) => &self.digit_6,
            WarriorId(6) => &self.digit_7,
            WarriorId(7) => &self.digit_8,
            #[allow(
                clippy::unreachable,
                reason = "All possible values of warrior_id are handled above."
            )]
            _ => unreachable!(),
        }
    }
}
