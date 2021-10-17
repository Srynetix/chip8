//! Status frame.

use chip8_core::drivers::{WINDOW_HEIGHT, WINDOW_WIDTH};
use macroquad::prelude::Rect;

use crate::{draw::ui_draw_text, frame::Frame};

/// Status height.
pub const STATUS_HEIGHT: u32 = 64;

/// Status frame.
pub struct StatusFrame {
    frame: Frame,
    status: String,
}

impl StatusFrame {
    /// Create new default frame.
    pub fn new_default() -> Self {
        Self {
            frame: Frame::new(
                Rect::new(
                    0.,
                    WINDOW_HEIGHT as f32 - STATUS_HEIGHT as f32,
                    WINDOW_WIDTH as f32,
                    STATUS_HEIGHT as f32,
                ),
                "STATUS",
            ),
            status: String::from(""),
        }
    }

    /// Set status message.
    pub fn set_status(&mut self, status: &str) {
        self.status = String::from(status);
    }

    /// Render frame.
    pub fn render(&self) {
        let font_size = 12;

        ui_draw_text(
            &self.status,
            self.frame.rect.x + 4.,
            self.frame.rect.y + font_size as f32 + 4.,
            font_size,
        );

        self.frame.render();
    }
}
