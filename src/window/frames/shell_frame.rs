//! Shell frame

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use time::PreciseTime;

use crate::core::error::CResult;
use crate::window::draw::{draw_text, DrawContext};
use crate::window::frame::Frame;

/// Shell frame
pub struct ShellFrame {
    frame: Frame,
    cmd_buffer: String,
    active: bool,
    blink_time: PreciseTime,
    cursor_shown: bool,
}

impl ShellFrame {
    /// Create new frame
    pub fn new(rect: Rect) -> Self {
        Self {
            frame: Frame::new(rect, "SHELL"),
            cmd_buffer: String::new(),
            active: false,
            blink_time: PreciseTime::now(),
            cursor_shown: false,
        }
    }

    /// Reset frame
    pub fn reset(&mut self) {
        self.cmd_buffer.clear();
        self.active = false;
        self.cursor_shown = false;
    }

    /// Set frame active
    pub fn set_active(&mut self, value: bool) {
        self.active = value;
    }

    /// Add char to command
    pub fn add_char(&mut self, ch: char) {
        self.cmd_buffer.push(ch);
    }

    /// Remove char from command
    pub fn remove_char(&mut self) {
        self.cmd_buffer.pop();
    }

    /// Validate command
    pub fn validate(&mut self) -> String {
        let cmd = self.cmd_buffer.clone();
        self.cmd_buffer.clear();

        cmd
    }

    /// Render frame
    pub fn render(&mut self, ctx: &mut DrawContext) -> CResult {
        let font = ctx.font_handler.get_or_create_font("default", 10).unwrap();
        let txt = format!("> {}", self.cmd_buffer);
        let sz = font.size_of(&txt)?;

        // Draw background
        let old_color = ctx.canvas.draw_color();
        ctx.canvas.set_draw_color(Color::RGB(0, 0, 0));
        ctx.canvas.fill_rect(self.frame.rect)?;
        ctx.canvas.set_draw_color(old_color);

        draw_text(
            ctx.canvas,
            ctx.texture_creator,
            font,
            &txt,
            self.frame.rect.x() as u32 + 4,
            self.frame.rect.y() as u32 + 4,
        )?;

        if self.active {
            if self.cursor_shown {
                draw_text(
                    ctx.canvas,
                    ctx.texture_creator,
                    font,
                    "_",
                    self.frame.rect.x() as u32 + 4 + sz.0,
                    self.frame.rect.y() as u32 + 4,
                )?;
            }

            let now = PreciseTime::now();
            if self.blink_time.to(now).num_milliseconds() >= 500 {
                self.blink_time = now;
                self.cursor_shown = !self.cursor_shown;
            }
        }

        self.frame.render(ctx)?;

        Ok(())
    }
}
