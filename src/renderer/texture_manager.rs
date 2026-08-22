use egui_macroquad::egui;
use macroquad::{prelude::ImageFormat, texture::Image};

use crate::warrior::warrior_id::WarriorId;

#[derive(Default)]
pub struct TextureManager {
    pub trophy: Option<egui::TextureHandle>,
    pub skull: Option<egui::TextureHandle>,
    digit_0: Option<egui::TextureHandle>,
    digit_1: Option<egui::TextureHandle>,
    digit_2: Option<egui::TextureHandle>,
    digit_3: Option<egui::TextureHandle>,
    digit_4: Option<egui::TextureHandle>,
    digit_5: Option<egui::TextureHandle>,
    digit_6: Option<egui::TextureHandle>,
    digit_7: Option<egui::TextureHandle>,
    digit_8: Option<egui::TextureHandle>,
}

/// Convert bytes to image, and register the image with the UI context.
/// Although bytes are available at compile time, registration with UI context must be done at runtime.
macro_rules! init_tex {
    ($self:ident, $egui_ctx:ident, $name:ident) => {
        $self.$name.get_or_insert_with(|| {
            // Load image to bytes at compile time.
            let bytes = include_bytes!(concat!("../../assets/images/", stringify!($name), ".png"));

            // Convert bytes to macroquad image.
            let mq_image = Image::from_file_with_format(bytes, Some(ImageFormat::Png)).unwrap();

            // Convert macroquad image to egui image.
            let size = [mq_image.width as usize, mq_image.height as usize];
            let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &mq_image.bytes);

            // Register this egui image with the UI context.
            $egui_ctx.load_texture(stringify!($name), color_image, Default::default())
        });
    };
}

impl TextureManager {
    /// Initialize all textures and register them with UI context.
    pub fn register_textures(&mut self, egui_ctx: &egui::Context) {
        init_tex!(self, egui_ctx, trophy);
        init_tex!(self, egui_ctx, skull);

        init_tex!(self, egui_ctx, digit_0);
        init_tex!(self, egui_ctx, digit_1);
        init_tex!(self, egui_ctx, digit_2);
        init_tex!(self, egui_ctx, digit_3);
        init_tex!(self, egui_ctx, digit_4);
        init_tex!(self, egui_ctx, digit_5);
        init_tex!(self, egui_ctx, digit_6);
        init_tex!(self, egui_ctx, digit_7);
        init_tex!(self, egui_ctx, digit_8);
    }

    pub fn get_warrior_icon(&self, warrior_id: WarriorId) -> Option<&egui::TextureHandle> {
        match warrior_id {
            0 => self.digit_0.as_ref(),
            1 => self.digit_1.as_ref(),
            2 => self.digit_2.as_ref(),
            3 => self.digit_3.as_ref(),
            4 => self.digit_4.as_ref(),
            5 => self.digit_5.as_ref(),
            6 => self.digit_6.as_ref(),
            7 => self.digit_7.as_ref(),
            _ => None,
        }
    }
}
