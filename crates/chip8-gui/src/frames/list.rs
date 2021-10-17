//! List frame.

use std::cmp;

use macroquad::prelude::Rect;

use crate::{draw::ui_draw_text, frame::Frame};

const LINES_MARGIN: usize = 3;

/// List frame.
pub struct ListFrame {
    frame: Frame,
}

/// List frame data.
pub struct ListFrameData<'a> {
    /// Cursor.
    pub cursor: i32,
    /// Data.
    pub data: &'a [String],
}

impl ListFrame {
    /// Create new list frame.
    pub fn new(rect: Rect, title: &str) -> Self {
        Self {
            frame: Frame::new(rect, title),
        }
    }

    fn get_max_lines(&self, font_size: u32) -> usize {
        let char_height = (font_size + 4) as usize;
        let rect_height = self.frame.rect.h as usize;

        (rect_height / char_height) - LINES_MARGIN
    }

    fn draw_data_count(&self, data: &ListFrameData) {
        let font_size = 8;
        let char_height = font_size;

        let cursor_x = self.frame.rect.x + 4.;
        let cursor_y = self.frame.rect.y + self.frame.rect.h - char_height as f32;
        let txt = &format!("{}/{} total", data.cursor + 1, data.data.len());

        ui_draw_text(txt, cursor_x, cursor_y, font_size as u16);
    }

    /// Render.
    pub fn render(&self, data: &ListFrameData) {
        let font_size = 8;
        let char_height = font_size + 4;
        let mut cursor_y = self.frame.rect.y + char_height as f32;
        let max_lines = self.get_max_lines(font_size);
        let total_lines = data.data.len();

        // Calculate start idx
        let start_idx = cmp::max(data.cursor + 1 - max_lines as i32, 0) as usize;
        let end_idx = cmp::min(total_lines, max_lines + start_idx) as usize;

        let mut count = start_idx;

        for elem in data.data[start_idx..end_idx].iter() {
            ui_draw_text(elem, self.frame.rect.x + 36., cursor_y, font_size as u16);

            if count == data.cursor as usize {
                ui_draw_text(">>>", self.frame.rect.x + 4., cursor_y, font_size as u16);
            }

            count += 1;
            cursor_y += char_height as f32;
        }

        self.draw_data_count(data);
        self.frame.render();
    }
}
