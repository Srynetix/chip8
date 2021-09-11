//! Title frame.

use crate::core::error::CResult;
use crate::rectf;
use crate::window::draw::{draw_text, DrawContext, Rect, WINDOW_WIDTH};
use crate::window::frame::Frame;

/// Title height.
pub const TITLE_HEIGHT: u32 = 64;

const PADDING: u32 = 24;

/// Title frame.
pub struct TitleFrame {
    frame: Frame,
    title: String,
}

impl TitleFrame {
    /// Create new frame.
    ///
    /// # Arguments
    ///
    /// * `rect` - Rect.
    /// * `title` - Title.
    ///
    /// # Returns
    ///
    /// * Title frame instance.
    ///
    pub fn new(rect: Rect, title: &str) -> Self {
        Self {
            frame: Frame::new(rect, "TITLE"),
            title: String::from(title),
        }
    }

    /// Create new default frame.
    ///
    /// # Arguments
    ///
    /// * `title` - Title.
    ///
    /// # Returns
    ///
    /// * Default title frame instance.
    ///
    pub fn new_default(title: &str) -> Self {
        Self {
            frame: Frame::new(rectf!(0, 0, WINDOW_WIDTH, TITLE_HEIGHT), "TITLE"),
            title: String::from(title),
        }
    }

    /// Set title.
    ///
    /// # Arguments
    ///
    /// * `title` - Title.
    ///
    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
    }

    /// Render frame.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Draw context
    ///
    /// # Returns
    ///
    /// * Result.
    ///
    pub fn render(&self, ctx: &mut DrawContext) -> CResult {
        let font = ctx.font_handler.get_or_create_font("default", 24).unwrap();
        let font_height = font.height();

        draw_text(
            ctx.canvas,
            ctx.texture_creator,
            font,
            &self.title,
            self.frame.rect.x() as u32 + PADDING,
            self.frame.rect.y() as u32 + (TITLE_HEIGHT / 2 as u32 - font_height as u32 / 2 as u32),
        )?;

        self.frame.render(ctx)?;

        Ok(())
    }
}
