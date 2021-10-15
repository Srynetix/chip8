use chip8_core::drivers::WINDOW_WIDTH;
use macroquad::prelude::Rect;

use crate::{draw::ui_draw_text, frame::Frame};

/// Title height.
pub const TITLE_HEIGHT: u32 = 64;

const PADDING: u32 = 24;

/// Title frame.
pub struct TitleFrame {
    frame: Frame,
    title: String,
}

impl TitleFrame {
    /// Create new default frame.
    pub fn new(title: &str) -> Self {
        Self {
            frame: Frame::new(
                Rect::new(0., 0., WINDOW_WIDTH as f32, TITLE_HEIGHT as f32),
                "TITLE",
            ),
            title: String::from(title),
        }
    }

    /// Set title.
    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
    }

    /// Render frame.
    pub fn render(&self) {
        let font_size = 48;
        let x_pos = self.frame.rect.x + PADDING as f32;
        let y_pos = self.frame.rect.y + TITLE_HEIGHT as f32 / 3.;

        ui_draw_text(&self.title, x_pos, y_pos, font_size);

        self.frame.render();
    }
}
