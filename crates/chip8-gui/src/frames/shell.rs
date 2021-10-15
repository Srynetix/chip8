//! Shell frame.

use std::{cmp, time::Instant};

use chip8_core::debugger::DebuggerStream;
use macroquad::prelude::Rect;

use crate::{
    draw::{ui_draw_fill_rect, ui_draw_text, ui_text_size},
    frame::Frame,
};

/// Shell frame.
pub struct ShellFrame {
    frame: Frame,
    cmd_buffer: String,
    active: bool,
    blink_time: Instant,
    cursor_shown: bool,
}

impl ShellFrame {
    /// Create new frame.
    pub fn new(rect: Rect) -> Self {
        Self {
            frame: Frame::new(rect, "SHELL"),
            cmd_buffer: String::new(),
            active: false,
            blink_time: Instant::now(),
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
    pub fn set_active(&mut self, value: bool) {
        self.active = value;
    }

    /// Add char to command.
    pub fn add_char(&mut self, ch: char) {
        self.cmd_buffer.push(ch);
    }

    /// Remove char from command.
    pub fn remove_char(&mut self) {
        self.cmd_buffer.pop();
    }

    /// Validate command.
    pub fn validate(&mut self) -> String {
        let cmd = self.cmd_buffer.clone();
        self.cmd_buffer.clear();

        cmd
    }

    /// Get max lines.
    pub fn get_max_lines(&self, font_size: u16) -> usize {
        let char_height = (font_size + 4) as usize;
        let rect_height = self.frame.rect.h as usize;

        rect_height / char_height
    }

    /// Render buffer.
    pub fn render_buffer(&mut self, stream: &DebuggerStream) {
        let font_size = 16;
        let mut cursor_y = self.frame.rect.y + 4.;
        let char_height = font_size as f32 + 4.;

        // Get max lines.
        let max_lines = self.get_max_lines(font_size);

        // Get buffer lines.
        let lines = stream.get_lines();
        let total_lines = lines.len();
        let base_cursor = total_lines as usize;

        let start_idx = cmp::max(base_cursor as i32 + 1 - max_lines as i32, 0) as usize;
        let end_idx = cmp::min(total_lines, max_lines + start_idx) as usize;

        for b in lines[start_idx..end_idx].iter() {
            ui_draw_text(&b.content, self.frame.rect.x + 4., cursor_y, font_size);

            cursor_y += char_height;
        }
    }

    /// Render frame.
    pub fn render(&mut self, stream: &DebuggerStream) {
        {
            // Draw background.
            ui_draw_fill_rect(self.frame.rect, macroquad::color::BLACK);
        }

        {
            // Draw stream content.
            self.render_buffer(stream);
        }

        {
            // Draw line buffer.
            let font_size = 16;
            let txt = format!("> {}", self.cmd_buffer);
            let sz = ui_text_size(&txt, font_size);
            let char_height = font_size as f32 + 4.;

            let cursor_y = char_height * stream.get_lines().len() as f32 + 4.;

            ui_draw_text(
                &txt,
                self.frame.rect.x + 4.,
                self.frame.rect.y + cursor_y,
                font_size,
            );

            if self.active {
                if self.cursor_shown {
                    ui_draw_text(
                        "_",
                        self.frame.rect.x + 4. + sz.width,
                        self.frame.rect.y + cursor_y,
                        font_size,
                    );
                }

                let now = Instant::now();
                if (now - self.blink_time).as_millis() > 500 {
                    self.blink_time = now;
                    self.cursor_shown = !self.cursor_shown;
                }
            }

            self.frame.render();
        }
    }
}
