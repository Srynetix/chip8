//! Memory frame

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::core::error::CResult;
use crate::emulator::Emulator;
use crate::peripherals::memory::CHUNK_SIZE;
use crate::window::draw::{draw_text, DrawContext};
use crate::window::frame::Frame;

/// Memory frame
pub struct MemoryFrame {
    frame: Frame,
}

impl MemoryFrame {
    /// Create new frame
    pub fn new(rect: Rect) -> Self {
        Self {
            frame: Frame::new(rect, "MMRY"),
        }
    }

    /// Render
    pub fn render(&self, emulator: &Emulator, ctx: &mut DrawContext) -> CResult {
        let font = ctx.font_handler.get_or_create_font("default", 6).unwrap();
        let mut output = String::new();

        // Draw background
        let old_color = ctx.canvas.draw_color();
        ctx.canvas.set_draw_color(Color::RGB(0, 0, 0));
        ctx.canvas.fill_rect(self.frame.rect)?;
        ctx.canvas.set_draw_color(old_color);

        for (idx, chunk) in emulator
            .cpu
            .peripherals
            .memory
            .get_data()
            .chunks(CHUNK_SIZE)
            .enumerate()
        {
            output.push_str(&format!(
                "{:04X}-{:04X}|",
                idx * CHUNK_SIZE,
                (idx + 1) * CHUNK_SIZE
            ));

            for chunk_value in chunk.iter() {
                output.push_str(&format!("{:02X}", chunk_value));
            }

            output.push('\n');
        }

        output.push_str(&format!(
            "PC: {:04X}",
            emulator.cpu.peripherals.memory.get_pointer()
        ));

        draw_text(
            ctx.canvas,
            ctx.texture_creator,
            font,
            &output,
            self.frame.rect.x() as u32 + 4,
            self.frame.rect.y() as u32 + 4,
        )?;

        self.frame.render(ctx)
    }
}
