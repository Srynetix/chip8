use macroquad::prelude::Rect;

use crate::draw::{ui_draw_frame, ui_draw_text, ui_text_size};

/// Frame.
pub struct Frame {
    /// Rect.
    pub rect: Rect,
    /// Title.
    pub title: String,
}

impl Frame {
    /// Create new frame.
    pub fn new(rect: Rect, title: &str) -> Self {
        Self {
            rect,
            title: String::from(title),
        }
    }

    /// Render frame.
    pub fn render(&self) {
        ui_draw_frame(self.rect);

        let font_size = 16.;
        let offset = font_size / 4.;
        let sz = ui_text_size(&self.title, font_size as u16);
        let x_pos = self.rect.x + self.rect.w - sz.width - offset;
        let y_pos = self.rect.y + offset;

        ui_draw_text(&self.title, x_pos, y_pos, font_size as u16);
    }
}
