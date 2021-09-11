//! Code frame.

use std::cmp;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::core::error::CResult;
use crate::core::types::C8Addr;
use crate::debugger::DebuggerContext;
use crate::peripherals::memory::INITIAL_MEMORY_POINTER;
use crate::window::draw::{draw_text, draw_text_ex, DrawContext};
use crate::window::font::Font;
use crate::window::frame::Frame;

/// Code frame.
pub struct CodeFrame {
    frame: Frame,
    buffer: Vec<String>,
}

impl CodeFrame {
    /// Create new frame.
    ///
    /// # Returns
    ///
    /// * Code frame instance.
    ///
    pub fn new(rect: Rect) -> Self {
        Self {
            frame: Frame::new(rect, "ASSEMBLY"),
            buffer: vec![],
        }
    }

    /// Reset.
    pub fn reset(&mut self) {
        self.buffer = vec![];
    }

    /// Get max lines.
    ///
    /// # Arguments
    ///
    /// * `font` - Font.
    ///
    /// # Returns
    ///
    /// * Max lines.
    ///s
    pub fn get_max_lines(&self, font: &Font) -> usize {
        let char_height = (font.height() + 4) as usize;
        let rect_height = self.frame.rect.height() as usize;

        rect_height / char_height
    }

    /// Add text.
    ///
    /// # Arguments
    ///
    /// * `text` - Text.
    ///
    pub fn add_text(&mut self, text: &str) {
        self.buffer.push(String::from(text))
    }

    /// Render frame.
    ///
    /// # Arguments
    ///
    /// * `debug_ctx` - Debug context.
    /// * `ctx` - Draw context.
    ///
    /// # Returns
    ///
    /// * Result.
    ///
    pub fn render(&self, debug_ctx: &DebuggerContext, ctx: &mut DrawContext) -> CResult {
        // If emulator is in continue mode, do not render code
        if debug_ctx.is_continuing {
            return self.frame.render(ctx);
        }

        let font = ctx.font_handler.get_font("default", 8).unwrap();
        let mut cursor_y = self.frame.rect.y() + 4;
        let char_height = font.height() + 4;

        let current_cursor = self.address_to_cursor(debug_ctx.address);

        let max_lines = self.get_max_lines(font);
        let total_lines = self.buffer.len();

        let start_idx = cmp::max(current_cursor + 1 - max_lines as i32, 0) as usize;
        let end_idx = cmp::min(total_lines, max_lines + start_idx) as usize;

        let mut count = start_idx;

        let grey_color = Color::RGB(127, 127, 127);
        let white_color = Color::RGB(255, 255, 255);

        for b in self.buffer[start_idx..end_idx].iter() {
            let color = if count == current_cursor as usize {
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

            if count == current_cursor as usize {
                draw_text(
                    ctx.canvas,
                    ctx.texture_creator,
                    font,
                    ">>>",
                    (self.frame.rect.x() + 56) as u32,
                    cursor_y as u32,
                )?;
            }

            // Breakpoint.
            if self.has_breakpoint_at_cursor(count as i32, debug_ctx) {
                draw_text(
                    ctx.canvas,
                    ctx.texture_creator,
                    font,
                    "B",
                    (self.frame.rect.x() + 44) as u32,
                    cursor_y as u32,
                )?;
            }

            count += 1;
            cursor_y += char_height;
        }

        self.frame.render(ctx)?;

        Ok(())
    }

    ///////////////
    // PRIVATE

    fn has_breakpoint_at_cursor(&self, cursor: i32, debug_ctx: &DebuggerContext) -> bool {
        for b in debug_ctx.breakpoints.0.iter() {
            let c = self.address_to_cursor(*b);
            if c == cursor {
                return true;
            }
        }

        false
    }

    fn address_to_cursor(&self, addr: C8Addr) -> i32 {
        i32::from(addr - INITIAL_MEMORY_POINTER) / 2
    }
}
