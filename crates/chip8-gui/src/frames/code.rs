//! Code frame.

use std::cmp;

use chip8_core::{
    core::types::C8Addr, debugger::DebuggerContext, peripherals::memory::INITIAL_MEMORY_POINTER,
};
use macroquad::prelude::Rect;

use crate::{
    draw::{ui_draw_text, ui_draw_text_ex},
    frame::Frame,
};

/// Code frame.
pub struct CodeFrame {
    frame: Frame,
    buffer: Vec<String>,
}

impl CodeFrame {
    /// Create new frame.
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
    pub fn get_max_lines(&self, font_size: u16) -> usize {
        let char_height = font_size as usize + 1;
        let rect_height = self.frame.rect.h as usize;

        rect_height / char_height - 1
    }

    /// Add text.
    pub fn add_text(&mut self, text: &str) {
        self.buffer.push(String::from(text))
    }

    /// Render frame.
    pub fn render(&self, debug_ctx: &DebuggerContext) {
        let font_size = 8;
        let mut cursor_y = self.frame.rect.y + font_size as f32 + 4.;
        let char_height = font_size as f32 + 1.;

        let current_cursor = self.address_to_cursor(debug_ctx.address);

        let max_lines = self.get_max_lines(font_size);
        let total_lines = self.buffer.len();

        let start_idx = cmp::max(current_cursor + 1 - max_lines as i32, 0) as usize;
        let end_idx = cmp::min(total_lines, max_lines + start_idx) as usize;

        let mut count = start_idx;

        let grey_color = macroquad::color::GRAY;
        let white_color = macroquad::color::WHITE;

        for b in self.buffer[start_idx..end_idx].iter() {
            let color = if count == current_cursor as usize {
                white_color
            } else {
                grey_color
            };

            ui_draw_text_ex(b, self.frame.rect.x + 4., cursor_y, font_size, color);

            if count == current_cursor as usize {
                ui_draw_text(">>>", self.frame.rect.x + 58., cursor_y, font_size);
            }

            // Breakpoint.
            if self.has_breakpoint_at_cursor(count as i32, debug_ctx) {
                ui_draw_text("B", self.frame.rect.x + 44., cursor_y, font_size);
            }

            count += 1;
            cursor_y += char_height;
        }

        self.frame.render();
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
