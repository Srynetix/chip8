//! List frame

use std::cmp;

use sdl2::rect::Rect;

use crate::core::error::CResult;
use crate::window::draw::{draw_text, DrawContext};
use crate::window::font::Font;
use crate::window::frame::Frame;

const LINES_MARGIN: usize = 3;

/// List frame
pub struct ListFrame {
    frame: Frame,
}

/// List frame data
pub struct ListFrameData<'a> {
    /// Cursor
    pub cursor: i32,
    /// Data
    pub data: &'a [String],
}

impl ListFrame {
    /// Create new list frame
    pub fn new(rect: Rect, title: &str) -> Self {
        Self {
            frame: Frame::new(rect, title),
        }
    }

    fn get_max_lines(&self, font: &Font) -> usize {
        let char_height = (font.height() + 4) as usize;
        let rect_height = self.frame.rect.height() as usize;

        (rect_height / char_height) - LINES_MARGIN
    }

    fn draw_data_count(&self, ctx: &mut DrawContext, data: &ListFrameData) -> CResult {
        let font = ctx.font_handler.get_font("default", 8).unwrap();
        let char_height = font.height() as u32 + 4 as u32;

        let cursor_x = self.frame.rect.x() + 4;
        let cursor_y = self.frame.rect.y() as u32 + self.frame.rect.height() - char_height;

        draw_text(
            ctx.canvas,
            ctx.texture_creator,
            font,
            &format!("{}/{} total", data.cursor + 1, data.data.len()),
            cursor_x as u32,
            cursor_y as u32,
        )
    }

    /// Render
    pub fn render(&self, ctx: &mut DrawContext, data: &ListFrameData) -> CResult {
        let font = ctx.font_handler.get_font("default", 8).unwrap();
        let char_height = font.height() + 4;
        let mut cursor_y = self.frame.rect.y() + char_height;
        let max_lines = self.get_max_lines(font);
        let total_lines = data.data.len();

        // Calculate start idx
        let start_idx = cmp::max(data.cursor + 1 - max_lines as i32, 0) as usize;
        let end_idx = cmp::min(total_lines, max_lines + start_idx) as usize;

        let mut count = start_idx;

        for elem in data.data[start_idx..end_idx].iter() {
            draw_text(
                ctx.canvas,
                ctx.texture_creator,
                font,
                elem,
                (self.frame.rect.x() + 36) as u32,
                cursor_y as u32,
            )?;

            if count == data.cursor as usize {
                draw_text(
                    ctx.canvas,
                    ctx.texture_creator,
                    font,
                    ">>>",
                    (self.frame.rect.x() + 4) as u32,
                    cursor_y as u32,
                )?;
            }

            count += 1;
            cursor_y += char_height;
        }

        self.draw_data_count(ctx, data)?;
        self.frame.render(ctx)?;

        Ok(())
    }
}
