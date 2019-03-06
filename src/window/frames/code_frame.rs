//! Code frame

use std::cmp;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::error::CResult;
use crate::window::draw::{draw_text, draw_text_ex, DrawContext};
use crate::window::font::Font;
use crate::window::frame::Frame;

/// Code frame
pub struct CodeFrame {
    frame: Frame,
    buffer: Vec<String>,
    cursor: i32,
}

impl CodeFrame {
    /// Create new frame
    pub fn new(rect: Rect) -> Self {
        Self {
            frame: Frame::new(rect, "ASSEMBLY"),
            buffer: vec![],
            cursor: 0,
        }
    }

    /// Get max lines
    pub fn get_max_lines(&self, font: &Font) -> usize {
        let char_height = (font.height() + 4) as usize;
        let rect_height = self.frame.rect.height() as usize;

        (rect_height / char_height)
    }

    /// Add text
    pub fn add_text(&mut self, text: &str) {
        self.buffer.push(String::from(text))
    }

    /// Render frame
    pub fn render(&self, ctx: &mut DrawContext) -> CResult {
        let font = ctx.font_handler.get_font("default", 8).unwrap();
        let mut cursor_y = self.frame.rect.y() + 4;
        let char_height = font.height() + 4;

        let max_lines = self.get_max_lines(font);
        let total_lines = self.buffer.len();

        let start_idx = cmp::max(self.cursor + 1 - max_lines as i32, 0) as usize;
        let end_idx = cmp::min(total_lines, max_lines + start_idx) as usize;

        let mut count = start_idx;

        let grey_color = Color::RGB(127, 127, 127);
        let white_color = Color::RGB(255, 255, 255);

        for b in self.buffer[start_idx..end_idx].iter() {
            let color = if count == self.cursor as usize {
                white_color
            } else {
                grey_color
            };

            draw_text_ex(
                ctx.canvas,
                ctx.texture_creator,
                font,
                b,
                (self.frame.rect.x() + 4) as u32,
                cursor_y as u32,
                color,
            )?;

            if count == self.cursor as usize {
                draw_text(
                    ctx.canvas,
                    ctx.texture_creator,
                    font,
                    ">>>",
                    (self.frame.rect.x() + 52) as u32,
                    cursor_y as u32,
                )?;
            }

            count += 1;
            cursor_y += char_height;
        }

        self.frame.render(ctx)?;

        Ok(())
    }
}
