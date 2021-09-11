//! Frame.

use sdl2::rect::Rect;

use crate::core::error::CResult;

use super::draw::{draw_frame, draw_text, DrawContext};

/// Frame.
pub struct Frame {
    /// Rect.
    pub rect: Rect,
    /// Title.
    pub title: String,
}

impl Frame {
    /// Create new frame.
    ///
    /// # Arguments
    ///
    /// * `rect` - Rect.
    /// * `title` - Title.
    ///
    /// # Returns
    ///
    /// * Frame instance.
    ///
    pub fn new(rect: Rect, title: &str) -> Self {
        Self {
            rect,
            title: String::from(title),
        }
    }

    /// Render frame.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Draw context.
    ///
    /// # Returns
    ///
    /// * Result.
    ///
    pub fn render(&self, ctx: &mut DrawContext) -> CResult {
        draw_frame(ctx.canvas, self.rect)?;

        let font = ctx.font_handler.get_font("default", 10).unwrap();
        let sz = font.size_of(&self.title).unwrap();
        let x_pos = self.rect.x() as u32 + self.rect.width() - sz.0 - 4 as u32;
        let y_pos = self.rect.y() as u32 + 4 as u32;

        draw_text(
            ctx.canvas,
            ctx.texture_creator,
            font,
            &self.title,
            x_pos,
            y_pos,
        )?;

        Ok(())
    }
}
