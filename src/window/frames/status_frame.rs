//! Status frame

use crate::error::CResult;
use crate::window::draw::{draw_text, DrawContext, Rect, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::window::frame::Frame;

const STATUS_HEIGHT: u32 = 64;

/// Status frame
pub struct StatusFrame {
    frame: Frame,
    status: String,
}

impl StatusFrame {
    /// Create new frame
    pub fn new(rect: Rect) -> Self {
        Self {
            frame: Frame::new(rect, "STATUS"),
            status: String::from(""),
        }
    }

    /// Create new default frame
    pub fn new_default() -> Self {
        Self {
            frame: Frame::new(
                rectf!(
                    0,
                    WINDOW_HEIGHT - STATUS_HEIGHT,
                    WINDOW_WIDTH,
                    STATUS_HEIGHT
                ),
                "STATUS",
            ),
            status: String::from(""),
        }
    }

    /// Set status message
    pub fn set_status(&mut self, status: &str) {
        self.status = String::from(status);
    }

    /// Render frame
    pub fn render(&self, ctx: &mut DrawContext) -> CResult {
        let font = ctx.font_handler.get_or_create_font("default", 10).unwrap();

        draw_text(
            ctx.canvas,
            ctx.texture_creator,
            font,
            &self.status,
            self.frame.rect.x() as u32 + 4 as u32,
            self.frame.rect.y() as u32 + 4 as u32,
        )?;

        self.frame.render(ctx)?;

        Ok(())
    }
}
