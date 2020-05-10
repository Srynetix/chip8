//! Shell frame.

use std::cmp;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use time::PreciseTime;

use crate::core::error::CResult;
use crate::debugger::DebuggerStream;
use crate::window::draw::{draw_text, DrawContext};
use crate::window::font::Font;
use crate::window::frame::Frame;

/// Shell frame.
pub struct ShellFrame {
    frame: Frame,
    cmd_buffer: String,
    active: bool,
    blink_time: PreciseTime,
    cursor_shown: bool,
}

impl ShellFrame {
    /// Create new frame.
    ///
    /// # Arguments
    ///
    /// * `rect` - Rect.
    ///
    /// # Returns
    ///
    /// * Shell frame instance.
    ///
    pub fn new(rect: Rect) -> Self {
        Self {
            frame: Frame::new(rect, "SHELL"),
            cmd_buffer: String::new(),
            active: false,
            blink_time: PreciseTime::now(),
            cursor_shown: false,
        }
    }

    /// Reset frame.
    pub fn reset(&mut self) {
        self.cmd_buffer.clear();
        self.active = false;
        self.cursor_shown = false;
    }

    /// Set frame active.
    ///
    /// # Arguments
    ///
    /// * `value` - Value.
    ///
    pub fn set_active(&mut self, value: bool) {
        self.active = value;
    }

    /// Add char to command.
    ///
    /// # Arguments
    ///
    /// `ch` - Character.
    ///
    pub fn add_char(&mut self, ch: char) {
        self.cmd_buffer.push(ch);
    }

    /// Remove char from command.
    pub fn remove_char(&mut self) {
        self.cmd_buffer.pop();
    }

    /// Validate command.
    ///
    /// # Returns
    ///
    /// * Command.
    ///
    pub fn validate(&mut self) -> String {
        let cmd = self.cmd_buffer.clone();
        self.cmd_buffer.clear();

        cmd
    }

    /// Get max lines.
    ///
    /// # Arguments
    ///
    /// * `font` - Font
    ///
    /// # Returns
    ///
    /// * Max lines.
    ///
    pub fn get_max_lines(&self, font: &Font) -> usize {
        let char_height = (font.height() + 4) as usize;
        let rect_height = self.frame.rect.height() as usize;

        rect_height / char_height
    }

    /// Render buffer.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Draw context
    /// * `stream` - Debugger stream
    ///
    /// # Returns
    ///
    /// * Result.
    ///
    pub fn render_buffer(&mut self, ctx: &mut DrawContext, stream: &DebuggerStream) -> CResult {
        let font = ctx.font_handler.get_or_create_font("default", 8).unwrap();
        let mut cursor_y = self.frame.rect.y() + 4;
        let char_height = font.height() + 4;

        // Get max lines.
        let max_lines = self.get_max_lines(font);

        // Get buffer lines.
        let lines = stream.get_lines();
        let total_lines = lines.len();
        let base_cursor = total_lines as usize;

        let start_idx = cmp::max(base_cursor as i32 + 1 - max_lines as i32, 0) as usize;
        let end_idx = cmp::min(total_lines, max_lines + start_idx) as usize;

        for b in lines[start_idx..end_idx].iter() {
            draw_text(
                ctx.canvas,
                ctx.texture_creator,
                font,
                &b.content,
                (self.frame.rect.x() + 4) as u32,
                cursor_y as u32,
            )?;

            cursor_y += char_height;
        }

        Ok(())
    }

    /// Render frame.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Draw context.
    /// * `stream` - Debugger stream.
    ///
    /// # Returns
    ///
    /// * Result.
    ///
    pub fn render(&mut self, ctx: &mut DrawContext, stream: &DebuggerStream) -> CResult {
        {
            // Draw background.
            let old_color = ctx.canvas.draw_color();
            ctx.canvas.set_draw_color(Color::RGB(0, 0, 0));
            ctx.canvas.fill_rect(self.frame.rect)?;
            ctx.canvas.set_draw_color(old_color);
        }

        {
            // Draw stream content.
            self.render_buffer(ctx, stream)?;
        }

        {
            // Draw line buffer.
            let font = ctx.font_handler.get_or_create_font("default", 8).unwrap();
            let txt = format!("> {}", self.cmd_buffer);
            let sz = font.size_of(&txt)?;
            let char_height = font.height() + 4;

            let cursor_y = char_height as u32 * stream.get_lines().len() as u32 + 4;

            draw_text(
                ctx.canvas,
                ctx.texture_creator,
                font,
                &txt,
                self.frame.rect.x() as u32 + 4,
                self.frame.rect.y() as u32 + cursor_y,
            )?;

            if self.active {
                if self.cursor_shown {
                    draw_text(
                        ctx.canvas,
                        ctx.texture_creator,
                        font,
                        "_",
                        self.frame.rect.x() as u32 + 4 + sz.0,
                        self.frame.rect.y() as u32 + cursor_y,
                    )?;
                }

                let now = PreciseTime::now();
                if self.blink_time.to(now).num_milliseconds() >= 500 {
                    self.blink_time = now;
                    self.cursor_shown = !self.cursor_shown;
                }
            }

            self.frame.render(ctx)?;
        }

        Ok(())
    }
}
